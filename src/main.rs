mod parser;
mod debug;

use std::fs;
use std::rc::Rc;
use std::path::PathBuf;
use std::io::Write;

use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::{common_token_stream::CommonTokenStream, token_factory::CommonTokenFactory};
use antlr_rust::InputStream;
use env_logger::{Builder, Target};
use log::LevelFilter;

use crate::debug::debugvisitor::DebugVisitor;
use crate::parser::cpp14parser::{TranslationUnitContextAll, CPP14ParserContextType};

// cargo build
// cargo run --bin build_parser
// cargo run --bin whatavr
//
// cargo run
//
// cargo fmt
fn main() {

    init_logging();
    log_start();

    log::info!("*************************************************\n");
    log::info!("Parser\n");
    log::info!("*************************************************\n");

    let mut file_path: String = String::new();
    file_path.push_str("test_resources/sample_files/cpp14/main.cpp");

    let srcdir = PathBuf::from(&file_path);
    log::info!("absolute path: {:?}\n", fs::canonicalize(&srcdir));

    let data = fs::read_to_string(&file_path).expect("Unable to read file");
    log::trace!("\n{}\n", data);

    let input_stream: InputStream<&str> = InputStream::new(data.as_str());

    //let token_factory: antlr_rust::token_factory::ArenaFactory<'_, antlr_rust::token_factory::CommonTokenFactory, antlr_rust::token::GenericToken<_>> = ArenaCommonFactory::default();
    let token_factory: CommonTokenFactory = CommonTokenFactory::default();
    
    let mut _lexer: parser::cpp14lexer::CPP14Lexer<'_, InputStream<&str>> = parser::cpp14lexer::CPP14Lexer::new_with_token_factory(
        input_stream,
        &token_factory,
    );
    let token_source: CommonTokenStream<'_, parser::cpp14lexer::CPP14Lexer<'_, InputStream<&str>>> = CommonTokenStream::new(_lexer);
    let mut parser: parser::cpp14parser::CPP14Parser<'_, CommonTokenStream<'_, parser::cpp14lexer::CPP14Lexer<'_, InputStream<&str>>>, antlr_rust::DefaultErrorStrategy<'_, CPP14ParserContextType>> = parser::cpp14parser::CPP14Parser::new(token_source);

    let result = parser.translationUnit();
    assert!(result.is_ok());
    let root: Rc<TranslationUnitContextAll> = result.unwrap();

    log::info!("*************************************************\n");
    log::info!("Visitor\n");
    log::info!("*************************************************\n");

    let mut visitor: DebugVisitor = DebugVisitor::default();

    let visitor_result = visitor.visit(&*root);
    log::trace!("{:?}\n", visitor_result);

    log_end();

}

fn init_logging() {
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Debug)
        // https://stackoverflow.com/questions/61810740/log-source-file-and-line-numbers
        .format(|buf, record| {
            //writeln!(
            write!(
                buf,
                //"{}:{} {} [{}] - {}",
                "{}:{} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                //chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

fn log_start() {
    log::trace!("Application starts ...\n");
    log::debug!("Application starts ...\n");
    log::info!("Application starts ...\n");
    log::warn!("Application starts ...\n");
    log::error!("Application starts ...\n");
}

fn log_end() {
    log::trace!("Application terminates.\n");
    log::debug!("Application terminates.\n");
    log::info!("Application terminates.\n");
    log::warn!("Application terminates.\n");
    log::error!("Application terminates.\n");
}