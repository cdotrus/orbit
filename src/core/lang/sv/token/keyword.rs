//
//  Copyright (C) 2022-2025  Chase Ruskin
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.
//

use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    AcceptOn,
    Alias,
    Always,
    AlwaysComb,
    AlwaysFf,
    AlwaysLatch,
    And,
    Assert,
    Assign,
    Assume,
    Automatic,
    Before,
    Begin,
    Bind,
    Bins,
    Binsof,
    Bit,
    Break,
    Buf,
    Bufif0,
    Bufif1,
    Byte,
    Case,
    Casex,
    Casez,
    Cell,
    Chandle,
    Checker,
    Class,
    Clocking,
    Cmos,
    Config,
    Const,
    Constraint,
    Context,
    Continue,
    Cover,
    Covergroup,
    Coverpoint,
    Cross,
    Deassign,
    Default,
    Defparam,
    Design,
    Disable,
    Dist,
    Do,
    Edge,
    Else,
    End,
    Endcase,
    Endchecker,
    Endclass,
    Endclocking,
    Endconfig,
    Endfunction,
    Endgenerate,
    Endgroup,
    Endinterface,
    Endmodule,
    Endpackage,
    Endprimitive,
    Endprogram,
    Endproperty,
    Endspecify,
    Endsequence,
    Endtable,
    Endtask,
    Enum,
    Event,
    Eventually,
    Expect,
    Export,
    Extends,
    Extern,
    Final,
    FirstMatch,
    For,
    Force,
    Foreach,
    Forever,
    Fork,
    Forkjoin,
    Function,
    Generate,
    Genvar,
    Global,
    Highz0,
    Highz1,
    If,
    Iff,
    Ifnone,
    IgnoreBins,
    IllegalBins,
    Implements,
    Implies,
    Import,
    Incdir,
    Include,
    Initial,
    Inout,
    Input,
    Inside,
    Instance,
    Int,
    Integer,
    Interconnect,
    Interface,
    Intersect,
    Join,
    JoinAny,
    JoinNone,
    Large,
    Let,
    Liblist,
    Library,
    Local,
    Localparam,
    Logic,
    Longint,
    Macromodule,
    Matches,
    Medium,
    Modport,
    Module,
    Nand,
    Negedge,
    Nettype,
    New,
    Nexttime,
    Nmos,
    Nor,
    Noshowcancelled,
    Not,
    Notif0,
    Notif1,
    Null,
    Or,
    Output,
    Package,
    Packed,
    Parameter,
    Pmos,
    Posedge,
    Primitive,
    Priority,
    Program,
    Property,
    Protected,
    Pull0,
    Pull1,
    Pulldown,
    Pullup,
    PulsestyleOndetect,
    PulsestyleOnevent,
    Pure,
    Rand,
    Randc,
    Randcase,
    Randsequence,
    Rcmos,
    Real,
    Realtime,
    Ref,
    Reg,
    RejectOn,
    Release,
    Repeat,
    Restrict,
    Return,
    Rnmos,
    Rpmos,
    Rtran,
    Rtranif0,
    Rtranif1,
    SAlways,
    SEventually,
    SNexttime,
    SUntil,
    SUntilWith,
    Scalared,
    Sequence,
    Shortint,
    Shortreal,
    Showcancelled,
    Signed,
    Small,
    Soft,
    Solve,
    Specify,
    Specparam,
    Static,
    String,
    Strong,
    Strong0,
    Strong1,
    Struct,
    Super,
    Supply0,
    Supply1,
    SyncAcceptOn,
    SyncRejectOn,
    Table,
    Tagged,
    Task,
    This,
    Throughout,
    Time,
    Timeprecision,
    Timeunit,
    Tran,
    Tranif0,
    Tranif1,
    Tri,
    Tri0,
    Tri1,
    Triand,
    Trior,
    Trireg,
    Type,
    Typedef,
    Union,
    Unique,
    Unique0,
    Unsigned,
    Until,
    UntilWith,
    Untyped,
    Use,
    Uwire,
    Var,
    Vectored,
    Virtual,
    Void,
    Wait,
    WaitOrder,
    Wand,
    Weak,
    Weak0,
    Weak1,
    While,
    Wildcard,
    Wire,
    With,
    Within,
    Wor,
    Xnor,
    Xor,
}

impl Keyword {
    /// Attempts to match the given string of characters `s` to a SystemVerilog keyword.
    ///
    /// Compares `s` against keywords using ascii comparison (case-sensitive).
    pub fn match_keyword(s: &str) -> Option<Self> {
        Some(match s.as_ref() {
            "accept_on" => Self::AcceptOn,
            "alias" => Self::Alias,
            "always" => Self::Always,
            "always_comb" => Self::AlwaysComb,
            "always_ff" => Self::AlwaysFf,
            "always_latch" => Self::AlwaysLatch,
            "and" => Self::And,
            "assert" => Self::Assert,
            "assign" => Self::Assign,
            "assume" => Self::Assume,
            "automatic" => Self::Automatic,
            "before" => Self::Before,
            "begin" => Self::Begin,
            "bind" => Self::Bind,
            "bins" => Self::Bins,
            "binsof" => Self::Binsof,
            "bit" => Self::Bit,
            "break" => Self::Break,
            "buf" => Self::Buf,
            "bufif0" => Self::Bufif0,
            "bufif1" => Self::Bufif1,
            "byte" => Self::Byte,
            "case" => Self::Case,
            "casex" => Self::Casex,
            "casez" => Self::Casez,
            "cell" => Self::Cell,
            "chandle" => Self::Chandle,
            "checker" => Self::Checker,
            "class" => Self::Class,
            "clocking" => Self::Clocking,
            "cmos" => Self::Cmos,
            "config" => Self::Config,
            "const" => Self::Const,
            "constraint" => Self::Constraint,
            "context" => Self::Context,
            "continue" => Self::Continue,
            "cover" => Self::Cover,
            "covergroup" => Self::Covergroup,
            "coverpoint" => Self::Coverpoint,
            "cross" => Self::Cross,
            "deassign" => Self::Deassign,
            "default" => Self::Default,
            "defparam" => Self::Defparam,
            "design" => Self::Design,
            "disable" => Self::Disable,
            "dist" => Self::Dist,
            "do" => Self::Do,
            "edge" => Self::Edge,
            "else" => Self::Else,
            "end" => Self::End,
            "endcase" => Self::Endcase,
            "endchecker" => Self::Endchecker,
            "endclass" => Self::Endclass,
            "endclocking" => Self::Endclocking,
            "endconfig" => Self::Endconfig,
            "endfunction" => Self::Endfunction,
            "endgenerate" => Self::Endgenerate,
            "endgroup" => Self::Endgroup,
            "endinterface" => Self::Endinterface,
            "endmodule" => Self::Endmodule,
            "endpackage" => Self::Endpackage,
            "endprimitive" => Self::Endprimitive,
            "endprogram" => Self::Endprogram,
            "endproperty" => Self::Endproperty,
            "endspecify" => Self::Endspecify,
            "endsequence" => Self::Endsequence,
            "endtable" => Self::Endtable,
            "endtask" => Self::Endtask,
            "enum" => Self::Enum,
            "event" => Self::Event,
            "eventually" => Self::Eventually,
            "expect" => Self::Expect,
            "export" => Self::Export,
            "extends" => Self::Extends,
            "extern" => Self::Extern,
            "final" => Self::Final,
            "first_match" => Self::FirstMatch,
            "for" => Self::For,
            "force" => Self::Force,
            "foreach" => Self::Foreach,
            "forever" => Self::Forever,
            "fork" => Self::Fork,
            "forkjoin" => Self::Forkjoin,
            "function" => Self::Function,
            "generate" => Self::Generate,
            "genvar" => Self::Genvar,
            "global" => Self::Global,
            "highz0" => Self::Highz0,
            "highz1" => Self::Highz1,
            "if" => Self::If,
            "iff" => Self::Iff,
            "ifnone" => Self::Ifnone,
            "ignore_bins" => Self::IgnoreBins,
            "illegal_bins" => Self::IllegalBins,
            "implements" => Self::Implements,
            "implies" => Self::Implies,
            "import" => Self::Import,
            "incdir" => Self::Incdir,
            "include" => Self::Include,
            "initial" => Self::Initial,
            "inout" => Self::Inout,
            "input" => Self::Input,
            "inside" => Self::Inside,
            "instance" => Self::Instance,
            "int" => Self::Int,
            "integer" => Self::Integer,
            "interconnect" => Self::Interconnect,
            "interface" => Self::Interface,
            "intersect" => Self::Intersect,
            "join" => Self::Join,
            "join_any" => Self::JoinAny,
            "join_none" => Self::JoinNone,
            "large" => Self::Large,
            "let" => Self::Let,
            "liblist" => Self::Liblist,
            "library" => Self::Library,
            "local" => Self::Local,
            "localparam" => Self::Localparam,
            "logic" => Self::Logic,
            "longint" => Self::Longint,
            "macromodule" => Self::Macromodule,
            "matches" => Self::Matches,
            "medium" => Self::Medium,
            "modport" => Self::Modport,
            "module" => Self::Module,
            "nand" => Self::Nand,
            "negedge" => Self::Negedge,
            "nettype" => Self::Nettype,
            "new" => Self::New,
            "nexttime" => Self::Nexttime,
            "nmos" => Self::Nmos,
            "nor" => Self::Nor,
            "noshowcancelled" => Self::Noshowcancelled,
            "not" => Self::Not,
            "notif0" => Self::Notif0,
            "notif1" => Self::Notif1,
            "null" => Self::Null,
            "or" => Self::Or,
            "output" => Self::Output,
            "package" => Self::Package,
            "packed" => Self::Packed,
            "parameter" => Self::Parameter,
            "pmos" => Self::Pmos,
            "posedge" => Self::Posedge,
            "primitive" => Self::Primitive,
            "priority" => Self::Priority,
            "program" => Self::Program,
            "property" => Self::Property,
            "protected" => Self::Protected,
            "pull0" => Self::Pull0,
            "pull1" => Self::Pull1,
            "pulldown" => Self::Pulldown,
            "pullup" => Self::Pullup,
            "pulsestyle_ondetect" => Self::PulsestyleOndetect,
            "pulsestyle_onevent" => Self::PulsestyleOnevent,
            "pure" => Self::Pure,
            "rand" => Self::Rand,
            "randc" => Self::Randc,
            "randcase" => Self::Randcase,
            "randsequence" => Self::Randsequence,
            "rcmos" => Self::Rcmos,
            "real" => Self::Real,
            "realtime" => Self::Realtime,
            "ref" => Self::Ref,
            "reg" => Self::Reg,
            "reject_on" => Self::RejectOn,
            "release" => Self::Release,
            "repeat" => Self::Repeat,
            "restrict" => Self::Restrict,
            "return" => Self::Return,
            "rnmos" => Self::Rnmos,
            "rpmos" => Self::Rpmos,
            "rtran" => Self::Rtran,
            "rtranif0" => Self::Rtranif0,
            "rtranif1" => Self::Rtranif1,
            "s_always" => Self::SAlways,
            "s_eventually" => Self::SEventually,
            "s_nexttime" => Self::SNexttime,
            "s_until" => Self::SUntil,
            "s_until_with" => Self::SUntilWith,
            "scalared" => Self::Scalared,
            "sequence" => Self::Sequence,
            "shortint" => Self::Shortint,
            "shortreal" => Self::Shortreal,
            "showcancelled" => Self::Showcancelled,
            "signed" => Self::Signed,
            "small" => Self::Small,
            "soft" => Self::Soft,
            "solve" => Self::Solve,
            "specify" => Self::Specify,
            "specparam" => Self::Specparam,
            "static" => Self::Static,
            "string" => Self::String,
            "strong" => Self::Strong,
            "strong0" => Self::Strong0,
            "strong1" => Self::Strong1,
            "struct" => Self::Struct,
            "super" => Self::Super,
            "supply0" => Self::Supply0,
            "supply1" => Self::Supply1,
            "sync_accept_on" => Self::SyncAcceptOn,
            "sync_reject_on" => Self::SyncRejectOn,
            "table" => Self::Table,
            "tagged" => Self::Tagged,
            "task" => Self::Task,
            "this" => Self::This,
            "throughout" => Self::Throughout,
            "time" => Self::Time,
            "timeprecision" => Self::Timeprecision,
            "timeunit" => Self::Timeunit,
            "tran" => Self::Tran,
            "tranif0" => Self::Tranif0,
            "tranif1" => Self::Tranif1,
            "tri" => Self::Tri,
            "tri0" => Self::Tri0,
            "tri1" => Self::Tri1,
            "triand" => Self::Triand,
            "trior" => Self::Trior,
            "trireg" => Self::Trireg,
            "type" => Self::Type,
            "typedef" => Self::Typedef,
            "union" => Self::Union,
            "unique" => Self::Unique,
            "unique0" => Self::Unique0,
            "unsigned" => Self::Unsigned,
            "until" => Self::Until,
            "until_with" => Self::UntilWith,
            "untyped" => Self::Untyped,
            "use" => Self::Use,
            "uwire" => Self::Uwire,
            "var" => Self::Var,
            "vectored" => Self::Vectored,
            "virtual" => Self::Virtual,
            "void" => Self::Void,
            "wait" => Self::Wait,
            "wait_order" => Self::WaitOrder,
            "wand" => Self::Wand,
            "weak" => Self::Weak,
            "weak0" => Self::Weak0,
            "weak1" => Self::Weak1,
            "while" => Self::While,
            "wildcard" => Self::Wildcard,
            "wire" => Self::Wire,
            "with" => Self::With,
            "within" => Self::Within,
            "wor" => Self::Wor,
            "xnor" => Self::Xnor,
            "xor" => Self::Xor,
            _ => return None,
        })
    }

    fn as_str(&self) -> &str {
        match self {
            Self::AcceptOn => "accept_on",
            Self::Alias => "alias",
            Self::Always => "always",
            Self::AlwaysComb => "always_comb",
            Self::AlwaysFf => "always_ff",
            Self::AlwaysLatch => "always_latch",
            Self::And => "and",
            Self::Assert => "assert",
            Self::Assign => "assign",
            Self::Assume => "assume",
            Self::Automatic => "automatic",
            Self::Before => "before",
            Self::Begin => "begin",
            Self::Bind => "bind",
            Self::Bins => "bins",
            Self::Binsof => "binsof",
            Self::Bit => "bit",
            Self::Break => "break",
            Self::Buf => "buf",
            Self::Bufif0 => "bufif0",
            Self::Bufif1 => "bufif1",
            Self::Byte => "byte",
            Self::Case => "case",
            Self::Casex => "casex",
            Self::Casez => "casez",
            Self::Cell => "cell",
            Self::Chandle => "chandle",
            Self::Checker => "checker",
            Self::Class => "class",
            Self::Clocking => "clocking",
            Self::Cmos => "cmos",
            Self::Config => "config",
            Self::Const => "const",
            Self::Constraint => "constraint",
            Self::Context => "context",
            Self::Continue => "continue",
            Self::Cover => "cover",
            Self::Covergroup => "covergroup",
            Self::Coverpoint => "coverpoint",
            Self::Cross => "cross",
            Self::Deassign => "deassign",
            Self::Default => "default",
            Self::Defparam => "defparam",
            Self::Design => "design",
            Self::Disable => "disable",
            Self::Dist => "dist",
            Self::Do => "do",
            Self::Edge => "edge",
            Self::Else => "else",
            Self::End => "end",
            Self::Endcase => "endcase",
            Self::Endchecker => "endchecker",
            Self::Endclass => "endclass",
            Self::Endclocking => "endclocking",
            Self::Endconfig => "endconfig",
            Self::Endfunction => "endfunction",
            Self::Endgenerate => "endgenerate",
            Self::Endgroup => "endgroup",
            Self::Endinterface => "endinterface",
            Self::Endmodule => "endmodule",
            Self::Endpackage => "endpackage",
            Self::Endprimitive => "endprimitive",
            Self::Endprogram => "endprogram",
            Self::Endproperty => "endproperty",
            Self::Endspecify => "endspecify",
            Self::Endsequence => "endsequence",
            Self::Endtable => "endtable",
            Self::Endtask => "endtask",
            Self::Enum => "enum",
            Self::Event => "event",
            Self::Eventually => "eventually",
            Self::Expect => "expect",
            Self::Export => "export",
            Self::Extends => "extends",
            Self::Extern => "extern",
            Self::Final => "final",
            Self::FirstMatch => "first_match",
            Self::For => "for",
            Self::Force => "force",
            Self::Foreach => "foreach",
            Self::Forever => "forever",
            Self::Fork => "fork",
            Self::Forkjoin => "forkjoin",
            Self::Function => "function",
            Self::Generate => "generate",
            Self::Genvar => "genvar",
            Self::Global => "global",
            Self::Highz0 => "highz0",
            Self::Highz1 => "highz1",
            Self::If => "if",
            Self::Iff => "iff",
            Self::Ifnone => "ifnone",
            Self::IgnoreBins => "ignore_bins",
            Self::IllegalBins => "illegal_bins",
            Self::Implements => "implements",
            Self::Implies => "implies",
            Self::Import => "import",
            Self::Incdir => "incdir",
            Self::Include => "include",
            Self::Initial => "initial",
            Self::Inout => "inout",
            Self::Input => "input",
            Self::Inside => "inside",
            Self::Instance => "instance",
            Self::Int => "int",
            Self::Integer => "integer",
            Self::Interconnect => "interconnect",
            Self::Interface => "interface",
            Self::Intersect => "intersect",
            Self::Join => "join",
            Self::JoinAny => "join_any",
            Self::JoinNone => "join_none",
            Self::Large => "large",
            Self::Let => "let",
            Self::Liblist => "liblist",
            Self::Library => "library",
            Self::Local => "local",
            Self::Localparam => "localparam",
            Self::Logic => "logic",
            Self::Longint => "longint",
            Self::Macromodule => "macromodule",
            Self::Matches => "matches",
            Self::Medium => "medium",
            Self::Modport => "modport",
            Self::Module => "module",
            Self::Nand => "nand",
            Self::Negedge => "negedge",
            Self::Nettype => "nettype",
            Self::New => "new",
            Self::Nexttime => "nexttime",
            Self::Nmos => "nmos",
            Self::Nor => "nor",
            Self::Noshowcancelled => "noshowcancelled",
            Self::Not => "not",
            Self::Notif0 => "notif0",
            Self::Notif1 => "notif1",
            Self::Null => "null",
            Self::Or => "or",
            Self::Output => "output",
            Self::Package => "package",
            Self::Packed => "packed",
            Self::Parameter => "parameter",
            Self::Pmos => "pmos",
            Self::Posedge => "posedge",
            Self::Primitive => "primitive",
            Self::Priority => "priority",
            Self::Program => "program",
            Self::Property => "property",
            Self::Protected => "protected",
            Self::Pull0 => "pull0",
            Self::Pull1 => "pull1",
            Self::Pulldown => "pulldown",
            Self::Pullup => "pullup",
            Self::PulsestyleOndetect => "pulsestyle_ondetect",
            Self::PulsestyleOnevent => "pulsestyle_onevent",
            Self::Pure => "pure",
            Self::Rand => "rand",
            Self::Randc => "randc",
            Self::Randcase => "randcase",
            Self::Randsequence => "randsequence",
            Self::Rcmos => "rcmos",
            Self::Real => "real",
            Self::Realtime => "realtime",
            Self::Ref => "ref",
            Self::Reg => "reg",
            Self::RejectOn => "reject_on",
            Self::Release => "release",
            Self::Repeat => "repeat",
            Self::Restrict => "restrict",
            Self::Return => "return",
            Self::Rnmos => "rnmos",
            Self::Rpmos => "rpmos",
            Self::Rtran => "rtran",
            Self::Rtranif0 => "rtranif0",
            Self::Rtranif1 => "rtranif1",
            Self::SAlways => "s_always",
            Self::SEventually => "s_eventually",
            Self::SNexttime => "s_nexttime",
            Self::SUntil => "s_until",
            Self::SUntilWith => "s_until_with",
            Self::Scalared => "scalared",
            Self::Sequence => "sequence",
            Self::Shortint => "shortint",
            Self::Shortreal => "shortreal",
            Self::Showcancelled => "showcancelled",
            Self::Signed => "signed",
            Self::Small => "small",
            Self::Soft => "soft",
            Self::Solve => "solve",
            Self::Specify => "specify",
            Self::Specparam => "specparam",
            Self::Static => "static",
            Self::String => "string",
            Self::Strong => "strong",
            Self::Strong0 => "strong0",
            Self::Strong1 => "strong1",
            Self::Struct => "struct",
            Self::Super => "super",
            Self::Supply0 => "supply0",
            Self::Supply1 => "supply1",
            Self::SyncAcceptOn => "sync_accept_on",
            Self::SyncRejectOn => "sync_reject_on",
            Self::Table => "table",
            Self::Tagged => "tagged",
            Self::Task => "task",
            Self::This => "this",
            Self::Throughout => "throughout",
            Self::Time => "time",
            Self::Timeprecision => "timeprecision",
            Self::Timeunit => "timeunit",
            Self::Tran => "tran",
            Self::Tranif0 => "tranif0",
            Self::Tranif1 => "tranif1",
            Self::Tri => "tri",
            Self::Tri0 => "tri0",
            Self::Tri1 => "tri1",
            Self::Triand => "triand",
            Self::Trior => "trior",
            Self::Trireg => "trireg",
            Self::Type => "type",
            Self::Typedef => "typedef",
            Self::Union => "union",
            Self::Unique => "unique",
            Self::Unique0 => "unique0",
            Self::Unsigned => "unsigned",
            Self::Until => "until",
            Self::UntilWith => "until_with",
            Self::Untyped => "untyped",
            Self::Use => "use",
            Self::Uwire => "uwire",
            Self::Var => "var",
            Self::Vectored => "vectored",
            Self::Virtual => "virtual",
            Self::Void => "void",
            Self::Wait => "wait",
            Self::WaitOrder => "wait_order",
            Self::Wand => "wand",
            Self::Weak => "weak",
            Self::Weak0 => "weak0",
            Self::Weak1 => "weak1",
            Self::While => "while",
            Self::Wildcard => "wildcard",
            Self::Wire => "wire",
            Self::With => "with",
            Self::Within => "within",
            Self::Wor => "wor",
            Self::Xnor => "xnor",
            Self::Xor => "xor",
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl serde::Serialize for Keyword {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
