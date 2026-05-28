use stratum::Stratum;
use clap::{Parser, Subcommand};
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Parser)]
#[command(
    name = "stratum",
    version = env!("CARGO_PKG_VERSION"),
    about = "Temporal-Semantic-Causal Database — query across time, meaning, and causality",
)]
struct Cli {
    #[arg(short, long, default_value = "./stratum_data")]
    db: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Start the Stratum HTTP API server
    Serve {
        #[arg(short, long, default_value = "127.0.0.1:7777")]
        addr: String,
    },
    /// Insert a record from the command line
    Insert {
        #[arg(short, long)]
        schema: String,
        #[arg(short, long)]
        data: String,
        #[arg(long)]
        causes: Option<String>,
    },
    /// Retrieve a record by hex ID
    Get { id: String },
    /// Execute an SQSL query
    Query { sqsl: String },
    /// Print database statistics
    Stats,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("stratum=info".parse()?)
        )
        .init();

    let cli = Cli::parse();
    let db = Arc::new(Stratum::open(&cli.db).await?);

    match cli.command {
        Command::Serve { addr } => {
            let addr: SocketAddr = addr.parse()?;
            let router = stratum::api::build_router(db.clone());

            println!();
            println!("  ┌─────────────────────────────────────────┐");
            println!("  │   S T R A T U M  v{}                 │", env!("CARGO_PKG_VERSION"));
            println!("  │   Temporal · Semantic · Causal Database  │");
            println!("  └─────────────────────────────────────────┘");
            println!();
            println!("  ● Listening  http://{}", addr);
            println!("  ● Database   {}", cli.db);
            println!("  ● Docs       https://vignesh2027.github.io/stratum");
            println!();

            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener, router).await?;
        }

        Command::Insert { schema, data, causes } => {
            let json_data: serde_json::Value = serde_json::from_str(&data)?;
            let mut builder = stratum::RecordBuilder::new()
                .schema(schema)
                .data(json_data);

            if let Some(causes_str) = causes {
                let mut cause_ids = Vec::new();
                for hex_id in causes_str.split(',').map(str::trim).filter(|s| !s.is_empty()) {
                    let bytes = hex::decode(hex_id)?;
                    if bytes.len() != 32 {
                        anyhow::bail!("cause ID must be 64 hex chars: {}", hex_id);
                    }
                    let mut id = [0u8; 32];
                    id.copy_from_slice(&bytes);
                    cause_ids.push(id);
                }
                builder = builder.caused_by(cause_ids);
            }

            let record = builder.build()?;
            let id = db.insert(record).await?;
            println!("{}", hex::encode(id));
        }

        Command::Get { id } => {
            let bytes = hex::decode(&id)?;
            if bytes.len() != 32 { anyhow::bail!("ID must be 64 hex chars"); }
            let mut rid = [0u8; 32];
            rid.copy_from_slice(&bytes);
            match db.get(&rid).await? {
                Some(record) => println!("{}", serde_json::to_string_pretty(&record)?),
                None => { eprintln!("Record not found: {}", id); std::process::exit(1); }
            }
        }

        Command::Query { sqsl } => {
            let records = db.query(&sqsl).await?;
            println!("Found {} record(s):", records.len());
            for record in records {
                println!("  {} [{}] {}", record.id_hex(), record.schema, record.data);
            }
        }

        Command::Stats => {
            let stats = db.stats().await;
            println!("Records     : {}", stats.total_records);
            println!("Time index  : {} timestamps", stats.temporal_entries);
            println!("Vectors     : {}", stats.semantic_vectors);
            println!("Causal edges: {}", stats.causal_edges);
        }
    }

    Ok(())
}
