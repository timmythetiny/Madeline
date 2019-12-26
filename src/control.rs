#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Kind {
    Integer,
    Float,
    Text,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Desc {
    name: &'static str,
    kind: Kind,
}

impl Desc {
    pub const fn new(name: &'static str, kind: Kind) -> Self {
        Self { name, kind }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Control {
    Integer(usize),
    Float(f32),
    Text(String),
}

impl From<&Kind> for Control {
    fn from(kind: &Kind) -> Self {
        match kind {
            Kind::Integer => Control::Integer(0),
            Kind::Float => Control::Float(0.0),
            Kind::Text => Control::Text(String::new()),
        }
    }
}

impl Control {
    pub fn as_int(&self) -> usize {
        match self {
            Control::Integer(value) => *value,
            _ => 0,
        }
    }

    pub fn as_float(&self) -> f32 {
        match self {
            Control::Float(value) => *value,
            _ => 0.0,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Control::Text(value) => value.as_str(),
            _ => "",
        }
    }
}
