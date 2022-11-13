use anyhow::{bail, Result};
use clap::ValueEnum;
use pallas::ledger::traverse::MultiEraHeader;
use pallas::network::miniprotocols::{
    blockfetch, chainsync, handshake, Point, MAINNET_MAGIC, PREVIEW_MAGIC, PRE_PRODUCTION_MAGIC,
};
use pallas::network::multiplexer::StdChannel;
use pallas::network::multiplexer::{bearers::Bearer, StdPlexer};
use tracing::{debug, info, warn};

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum Network {
    Preview,
    Preprod,
    Mainnet,
}

pub struct Ouroboros {
    chainsync: chainsync::N2NClient<StdChannel>,
    blockfetch: blockfetch::Client<StdChannel>,
}

impl Ouroboros {
    pub fn new(network: Network) -> Result<Ouroboros> {
        // setup a TCP socket to act as data bearer between our agents and the remote
        // relay.
        let (bearer, magic) = match network {
            Network::Preview => (
                Bearer::connect_tcp("preview-node.world.dev.cardano.org:30002")?,
                PREVIEW_MAGIC,
            ),
            Network::Preprod => (
                Bearer::connect_tcp("preprod-node.world.dev.cardano.org:30000")?,
                PRE_PRODUCTION_MAGIC,
            ),
            Network::Mainnet => (
                Bearer::connect_tcp("relays-new.cardano-mainnet.iohk.io:3001")?,
                MAINNET_MAGIC,
            ),
        };

        // setup the multiplexer by specifying the bearer and the IDs of the
        // miniprotocols to use
        let mut plexer = StdPlexer::new(bearer);
        let channel0 = plexer.use_channel(0);
        let channel3 = plexer.use_channel(3);
        let channel2 = plexer.use_channel(2);

        plexer.muxer.spawn();
        plexer.demuxer.spawn();

        // execute the required handshake against the relay
        let mut client = handshake::N2NClient::new(channel0);

        let confirmation = client
            .handshake(handshake::n2n::VersionTable::v7_and_above(magic))
            .unwrap();

        match confirmation {
            handshake::Confirmation::Accepted(v, _) => {
                debug!("hand-shake accepted, using version {}", v);
            }
            handshake::Confirmation::Rejected(x) => {
                bail!("hand-shake rejected with reason {:?}", x)
            }
        }

        Ok(Ouroboros {
            chainsync: chainsync::Client::new(channel2),
            blockfetch: blockfetch::Client::new(channel3),
        })
    }

    pub fn get_tip(&mut self) -> Result<Point, chainsync::Error> {
        self.chainsync.intersect_tip()
    }

    pub fn get_block_bytes(&mut self, point: Point) -> Result<Vec<u8>, blockfetch::Error> {
        let block_bytes = self.blockfetch.fetch_single(point.clone())?;

        Ok(block_bytes)
    }

    pub fn get_next_block_bytes(&mut self) -> Result<Vec<u8>> {
        loop {
            let msg = match self.chainsync.has_agency() {
                true => self.chainsync.request_next()?,
                false => self.chainsync.recv_while_can_await()?,
            };

            match msg {
                chainsync::NextResponse::RollForward(h, _) => {
                    let header =
                        MultiEraHeader::decode(h.variant, h.byron_prefix.map(|x| x.0), &h.cbor)?;

                    let point = Point::Specific(header.slot(), header.hash().to_vec());

                    debug!("chainsync rollforward ({:?})", point);

                    return Ok(self.get_block_bytes(point)?);
                }
                chainsync::NextResponse::RollBackward(p, _) => {
                    warn!("chainsync rollback ({:?})", p);
                }
                chainsync::NextResponse::Await => {
                    info!("awaiting next block...");
                }
            }
        }
    }
}
