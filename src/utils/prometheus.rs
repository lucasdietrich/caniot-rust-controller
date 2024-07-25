use std::fmt::Display;

pub trait PrometheusLabel: Display {
    fn label(&self) -> String {
        format!("{}", self)
    }
}

impl<T> PrometheusLabel for T where T: Display {}

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
pub fn join_labels<T: PrometheusLabel>(labels: impl AsRef<[T]>) -> String {
    labels
        .as_ref()
        .iter()
        .map(|l| l.label())
        .collect::<Vec<_>>()
        .join(",")
}

pub trait PrometheusExporterTrait {
    type Label: PrometheusLabel;

    fn export(&self, labels: impl AsRef<[Self::Label]>) -> String;
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use crate::utils::{join_labels, PrometheusExporterTrait};

    #[test]
    fn test_prometheus_labels() {
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

        let labels = vec![
            Label::Medium("BLE".to_string()),
            Label::Mac("A4:C1:38:68:05:63".to_string()),
        ];

        struct Stats {
            rx: usize,
            tx: usize,
        }

        impl PrometheusExporterTrait for Stats {
            type Label = Label;

            fn export(&self, labels: impl AsRef<[Self::Label]>) -> String {
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
