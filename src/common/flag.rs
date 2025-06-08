use std::fmt::Display;

use log::{info, warn, error};

pub enum Flag {
    None,
    Info {
        kind: FlagKind,
        info: String
    },
    Warn {
        kind: FlagKind,
        warn: String
    },
    Error {
        kind: FlagKind,
        error: String
    }
}

pub enum FlagKind {
    Lexical,
    Syntactical,
    Semantic
}

impl Display for FlagKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &FlagKind::Lexical => write!(f, "lexical"),
            &FlagKind::Syntactical => write!(f, "syntactical"),
            &FlagKind::Semantic => write!(f, "semantic"),
        }
    }
}

impl Flag {
    pub fn log(&self) {
        match &self {
            &Flag::None => return,
            &Flag::Info { kind, info } => {
                info!("{} info flag: {}", kind, info)
            },
            &Flag::Warn { kind, warn } => {
                warn!("{} warn flag: {}", kind, warn)
            },
            &Flag::Error { kind, error } => {
                error!("{} error flag: {}", kind, error)
            }
        }
    }
}
