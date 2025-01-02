use std::fmt::Display;

pub trait PrometheusLabelTrait: Display + Clone {
    fn label(&self) -> String {
        format!("{}", self)
    }
}

impl<L> PrometheusLabelTrait for L where L: Display + Clone {}

#[derive(Clone)]
pub enum PrometheusNoLabel {}

impl Display for PrometheusNoLabel {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[macro_export]
macro_rules! impl_display_for_enum {
    ($enum_name:ident { $($variant:ident($type:ty)),* $(,)? }) => {
        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        $enum_name::$variant(value) => write!(f, "{}=\"{}\"", stringify!($variant).to_lowercase(), value),
                    )*
                }
            }
        }
    }
}

// TODO: Include the start "{" and end "}" immediately, don't include them if the labels are empty
pub fn join_labels<'a, L: PrometheusLabelTrait + 'a>(labels: impl AsRef<[&'a L]>) -> String {
    labels
        .as_ref()
        .iter()
        .map(|l| l.label())
        .collect::<Vec<_>>()
        .join(",")
}

pub trait PrometheusExporterTrait<'a> {
    type Label: PrometheusLabelTrait + 'a;

    fn export(&self, labels: impl AsRef<[&'a Self::Label]>) -> String;
}

pub fn format_metric<'a>(
    name: &str,
    value: impl Display,
    labels: impl AsRef<[&'a SensorLabel]>,
) -> String {
    let labels = labels.as_ref();
    if labels.is_empty() {
        format!("{} {}", name, value)
    } else {
        format!("{}{{{}}} {}", name, join_labels(labels), value)
    }
}

#[derive(Clone)]
pub enum DeviceLabel {
    Name(String),
    Controller(String),
    Medium(String),
    Mac(String),
    Class(u8),
    SubId(u8),
}

impl_display_for_enum!(DeviceLabel { Name(String), Controller(String), Medium(String), Mac(String), Class(String), SubId(String) });

// prometheus library
#[derive(Clone)]
pub enum SensorLabel {
    Controller(String), // alarm, other
    Install(String),    // indoor, outdoor
    Location(String),   // south, east
}

impl_display_for_enum!(SensorLabel { Controller(String), Install(String), Location(String) });

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use crate::utils::{join_labels, PrometheusExporterTrait};

    #[test]
    fn test_prometheus_labels() {
        #[derive(Clone)]
        enum Label {
            Medium(String),
            Mac(String),
        }

        impl Display for Label {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Label::Medium(m) => write!(f, "medium=\"{}\"", m),
                    Label::Mac(m) => write!(f, "mac=\"{}\"", m),
                }
            }
        }

        let label_ble = Label::Medium("BLE".to_string());
        let label_mac = Label::Mac("A4:C1:38:68:05:63".to_string());
        let labels = vec![&label_ble, &label_mac];

        struct Stats {
            rx: usize,
            tx: usize,
        }

        impl<'a> PrometheusExporterTrait<'a> for Stats {
            type Label = Label;

            fn export(&self, labels: impl AsRef<[&'a Self::Label]>) -> String {
                let labels = join_labels(labels);
                format!(
                    "can_rx {{{labels}}} {}\n\
                    can_tx {{{labels}}} {}\n",
                    self.rx, self.tx
                )
            }
        }

        let stats = Stats { rx: 10, tx: 20 };

        let exported = stats.export(&labels);
        assert_eq!(exported, "can_rx {medium=\"BLE\",mac=\"A4:C1:38:68:05:63\"} 10\ncan_tx {medium=\"BLE\",mac=\"A4:C1:38:68:05:63\"} 20\n");
    }
}
