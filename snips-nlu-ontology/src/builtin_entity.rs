use std::ops::Range;

use serde::Deserialize;
use serde_json;

use errors::*;
use language::Language;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BuiltinEntity {
    pub value: String,
    pub range: Range<usize>,
    pub entity: ::SlotValue,
    #[serde(serialize_with = "serialize_builtin_entity_kind",
            deserialize_with = "deserialize_builtin_entity_kind")]
    pub entity_kind: BuiltinEntityKind,
}

fn serialize_builtin_entity_kind<S>(
    value: &BuiltinEntityKind,
    serializer: S,
) -> ::std::result::Result<S::Ok, S::Error>
where
    S: ::serde::Serializer,
{
    serializer.serialize_str(value.identifier())
}

fn deserialize_builtin_entity_kind<'de, D>(
    deserializer: D,
) -> ::std::result::Result<BuiltinEntityKind, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
    String::deserialize(deserializer)
        .and_then(|s| BuiltinEntityKind::from_identifier(&s).map_err(::serde::de::Error::custom))
}

enum_kind!(
    BuiltinEntityKind,
    [
        AmountOfMoney,
        Duration,
        Number,
        Ordinal,
        Temperature,
        Time,
        Percentage
    ]
);

impl BuiltinEntityKind {
    pub fn identifier(&self) -> &str {
        match *self {
            BuiltinEntityKind::AmountOfMoney => "snips/amountOfMoney",
            BuiltinEntityKind::Duration => "snips/duration",
            BuiltinEntityKind::Number => "snips/number",
            BuiltinEntityKind::Ordinal => "snips/ordinal",
            BuiltinEntityKind::Temperature => "snips/temperature",
            BuiltinEntityKind::Time => "snips/datetime",
            BuiltinEntityKind::Percentage => "snips/percentage",
        }
    }

    pub fn from_identifier(identifier: &str) -> Result<Self> {
        BuiltinEntityKind::all()
            .iter()
            .find(|kind| kind.identifier() == identifier)
            .map(|k| k.clone())
            .ok_or(format_err!("Unknown EntityKind identifier: {}", identifier))
    }
}

impl BuiltinEntityKind {
    pub fn description(&self) -> &str {
        match *self {
            BuiltinEntityKind::AmountOfMoney => "Matches amount of money",
            BuiltinEntityKind::Duration => "Matches time duration",
            BuiltinEntityKind::Number => "Matches a cardinal numbers",
            BuiltinEntityKind::Ordinal => "Matches a ordinal numbers",
            BuiltinEntityKind::Temperature => "Matches a temperature",
            BuiltinEntityKind::Time => "Matches date, time, intervals or date and time together",
            BuiltinEntityKind::Percentage => "Matches a percentage",
        }
    }
}

impl BuiltinEntityKind {
    pub fn examples(&self, language: Language) -> &[&str] {
        match language {
            Language::DE => self.de_examples(),
            Language::EN => self.en_examples(),
            Language::ES => self.es_examples(),
            Language::FR => self.fr_examples(),
            Language::JA => self.ja_examples(),
            Language::KO => self.ko_examples(),
        }
    }

    fn de_examples(&self) -> &[&str] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &[
                "10$",
                "ungefähr 5€",
                "zwei tausend Dollar",
            ],
            BuiltinEntityKind::Duration => &[
                "2stdn",
                "drei monate",
                "ein halbe Stunde",
                "8 Jahre und zwei Tage",
            ],
            BuiltinEntityKind::Number => &[
                "2001",
                "einundzwanzig",
                "zwei tausend",
                "zwei tausend und drei"
            ],
            BuiltinEntityKind::Ordinal => &[
                "Erste",
                "der zweite",
                "zwei und zwanzigster"
            ],
            BuiltinEntityKind::Temperature => &[
                "70K",
                "3°C",
                "Dreiundzwanzig Grad",
                "zweiunddreißig Grad Fahrenheit",
            ],
            BuiltinEntityKind::Time => &[
                "Heute",
                "16.30 Uhr",
                "in 1 Stunde",
                "dritter Dienstag im Juni",
            ],
            BuiltinEntityKind::Percentage => &[
                "25%",
                "zwanzig Prozent",
                "zwei tausend und fünfzig Prozent",
            ],
        }
    }

    fn en_examples(&self) -> &[&str] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &[
                "10$",
                "around 5€",
                "ten dollars and five cents",
            ],
            BuiltinEntityKind::Duration => &[
                "1h",
                "3 months",
                "half an hour",
                "8 years and two days",
            ],
            BuiltinEntityKind::Number => &[
                "2001",
                "twenty one",
                "three hundred and four",
            ],
            BuiltinEntityKind::Ordinal => &[
                "1st",
                "the second",
                "the twenty third",
            ],
            BuiltinEntityKind::Temperature => &[
                "70K",
                "3°C",
                "Twenty three degrees",
                "one hundred degrees fahrenheit",
            ],
            BuiltinEntityKind::Time => &[
                "Today",
                "4:30 pm",
                "in 1 hour",
                "3rd tuesday of June",
            ],
            BuiltinEntityKind::Percentage => &[
                "25%",
                "twenty percent",
                "two hundred and fifty percents",
            ],
        }
    }

    fn es_examples(&self) -> &[&str] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &[
                "10$",
                "cinco euros",
                "diez dólares y cinco centavos",
            ],
            BuiltinEntityKind::Duration => &[
                "1h",
                "3 meses",
                // TODO: Add these examples when they are supported by the BuiltinEntityParser
                // "ciento dos minutos",
                // "8 años y dos dias",
            ],
            BuiltinEntityKind::Number => &[
                "2001",
                "diez y ocho",
                // TODO: Add these examples when they are supported by the BuiltinEntityParser
                // "ciento dos",
                // "tres mil nueve",
                // "ciento cuarenta y nueve",
            ],
            BuiltinEntityKind::Ordinal => &[
                "primer",
                // TODO: Add these examples when they are supported by the BuiltinEntityParser
                // "vigésimo primero",
            ],
            BuiltinEntityKind::Temperature => &[
                "70K",
                "3°C",
                "veintitrés grados",
                // TODO: Add these examples when they are supported by the BuiltinEntityParser
                // "tres mil grados Fahrenheit",
            ],
            BuiltinEntityKind::Time => &[
                "hoy",
                "esta noche",
                "a la 1:30",
                "el primer jueves de junio",
            ],
            BuiltinEntityKind::Percentage => &[
                "25%",
                "quince porcientos",
                "20 por ciento",
                // TODO: Add these examples when they are supported by the BuiltinEntityParser
                // "tres mil por ciento",
            ],
        }
    }

    fn fr_examples(&self) -> &[&str] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &[
                "10$",
                "environ 5€",
                "dix dollars et cinq centimes",
            ],
            BuiltinEntityKind::Duration => &[
                "1h",
                "3 mois",
                "une demi heure",
                "8 ans et deux jours",
            ],
            BuiltinEntityKind::Number => &[
                "2001",
                "vingt deux",
                "deux cent trois",
                "quatre vingt dix neuf"
            ],
            BuiltinEntityKind::Ordinal => &[
                "1er",
                "le deuxième",
                "vingt et unieme",
            ],
            BuiltinEntityKind::Temperature => &[
                "70K",
                "3°C",
                "vingt trois degrés",
                "deux cent degrés Fahrenheit",
            ],
            BuiltinEntityKind::Time => &[
                "Aujourd'hui",
                "à 14:30",
                "dans 1 heure",
                "le premier jeudi de Juin",
            ],
            BuiltinEntityKind::Percentage => &[
                "25%",
                "20 pourcents",
                "quatre vingt dix pourcents",
            ],
        }
    }

    fn ja_examples(&self) -> &[&str] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &[],
            BuiltinEntityKind::Duration => &[],
            BuiltinEntityKind::Number => &[
                "十二",
                "二千五",
                "四千三百二",
            ],
            BuiltinEntityKind::Ordinal => &[],
            BuiltinEntityKind::Temperature => &[],
            BuiltinEntityKind::Time => &[],
            BuiltinEntityKind::Percentage => &[],
        }
    }

    fn ko_examples(&self) -> &[&str] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &[
                "10$",
                "약 5 유로",
                "10 달러 5 센트",
            ],
            BuiltinEntityKind::Duration => &[
                "양일",
                "1시간",
                "3 개월",
            ],
            BuiltinEntityKind::Number => &[
                "2001",
                "삼천",
                "스물 둘",
                "천 아흔 아홉",
            ],
            BuiltinEntityKind::Ordinal => &[
                "첫",
                "첫번째"
            ],
            BuiltinEntityKind::Temperature => &[
                "5도",
                "섭씨 20도",
                "화씨 백 도",
            ],
            BuiltinEntityKind::Time => &[
                "오늘",
                "14시 30 분에",
                "5 월 첫째 목요일",
            ],
            BuiltinEntityKind::Percentage => &[],
        }
    }
}

impl BuiltinEntityKind {
    pub fn result_description(&self) -> Result<String> {
        Ok(match *self {
            BuiltinEntityKind::AmountOfMoney => serde_json::to_string_pretty(&vec![
                ::SlotValue::AmountOfMoney(::AmountOfMoneyValue {
                    value: 10.05,
                    precision: ::Precision::Approximate,
                    unit: Some("€".to_string()),
                }),
            ]),
            BuiltinEntityKind::Duration => serde_json::to_string_pretty(&vec![
                ::SlotValue::Duration(::DurationValue {
                    years: 0,
                    quarters: 0,
                    months: 3,
                    weeks: 0,
                    days: 0,
                    hours: 0,
                    minutes: 0,
                    seconds: 0,
                    precision: ::Precision::Exact,
                }),
            ]),
            BuiltinEntityKind::Number => serde_json::to_string_pretty(&vec![
                ::SlotValue::Number(::NumberValue { value: 42. }),
            ]),
            BuiltinEntityKind::Ordinal => serde_json::to_string_pretty(&vec![
                ::SlotValue::Ordinal(::OrdinalValue { value: 2 }),
            ]),
            BuiltinEntityKind::Temperature => serde_json::to_string_pretty(&vec![
                ::SlotValue::Temperature(::TemperatureValue {
                    value: 23.0,
                    unit: Some("celsius".to_string()),
                }),
                ::SlotValue::Temperature(::TemperatureValue {
                    value: 60.0,
                    unit: Some("fahrenheit".to_string()),
                }),
            ]),
            BuiltinEntityKind::Time => serde_json::to_string_pretty(&vec![
                ::SlotValue::InstantTime(::InstantTimeValue {
                    value: "2017-06-13 18:00:00 +02:00".to_string(),
                    grain: ::Grain::Hour,
                    precision: ::Precision::Exact,
                }),
                ::SlotValue::TimeInterval(::TimeIntervalValue {
                    from: Some("2017-06-07 18:00:00 +02:00".to_string()),
                    to: Some("2017-06-08 00:00:00 +02:00".to_string()),
                }),
            ]),
            BuiltinEntityKind::Percentage => serde_json::to_string_pretty(&vec![
                ::SlotValue::Percentage(::PercentageValue { value: 20. }),
            ]),
        }?)
    }
}

impl BuiltinEntityKind {
    pub fn supported_languages(&self) -> &[Language] {
        match *self {
            BuiltinEntityKind::AmountOfMoney => &[
                Language::DE,
                Language::EN,
                Language::ES,
                Language::FR,
                Language::JA,
                Language::KO,
            ],
            BuiltinEntityKind::Duration => &[
                Language::DE,
                Language::EN,
                Language::ES,
                Language::FR,
                Language::JA,
                Language::KO,
            ],
            BuiltinEntityKind::Number => &[
                Language::DE,
                Language::EN,
                Language::ES,
                Language::FR,
                Language::JA,
                Language::KO,
            ],
            BuiltinEntityKind::Ordinal => &[
                Language::DE,
                Language::EN,
                Language::ES,
                Language::FR,
                Language::JA,
                Language::KO,
            ],
            BuiltinEntityKind::Temperature => &[
                Language::DE,
                Language::EN,
                Language::ES,
                Language::FR,
                Language::JA,
                Language::KO,
            ],
            BuiltinEntityKind::Time => &[
                Language::DE,
                Language::EN,
                Language::ES,
                Language::FR,
                Language::JA,
                Language::KO,
            ],
            BuiltinEntityKind::Percentage => &[
                Language::DE,
                Language::EN,
                Language::ES,
                Language::FR,
                Language::JA,
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_result_descriptions() {
        // Given
        let description = BuiltinEntityKind::Percentage.result_description().unwrap();

        // When/Then
        let expected_description =
            "[\n  {\n    \"kind\": \"Percentage\",\n    \"value\": 20.0\n  }\n]";
        assert_eq!(expected_description, description);
    }

    #[test]
    fn test_builtin_entity_ser_de() {
        let entity = BuiltinEntity {
            value: "hello".to_string(),
            range: 12..42,
            entity: ::SlotValue::InstantTime(::InstantTimeValue {
                value: "some_value".into(),
                grain: ::Grain::Year,
                precision: ::Precision::Exact,
            }),
            entity_kind: BuiltinEntityKind::Time,
        };

        assert_tokens(
            &entity,
            &[
                Token::Struct {
                    name: "BuiltinEntity",
                    len: 4,
                },
                Token::Str("value"),
                Token::Str("hello"),
                Token::Str("range"),
                Token::Struct {
                    name: "Range",
                    len: 2,
                },
                Token::Str("start"),
                Token::U64(12),
                Token::Str("end"),
                Token::U64(42),
                Token::StructEnd,
                Token::Str("entity"),
                Token::Struct {
                    name: "InstantTimeValue",
                    len: 4,
                },
                Token::Str("kind"),
                Token::Str("InstantTime"),
                Token::Str("value"),
                Token::String("some_value"),
                Token::Str("grain"),
                Token::UnitVariant {
                    name: "Grain",
                    variant: "Year",
                },
                Token::Str("precision"),
                Token::UnitVariant {
                    name: "Precision",
                    variant: "Exact",
                },
                Token::StructEnd,
                Token::Str("entity_kind"),
                Token::Str("snips/datetime"),
                Token::StructEnd,
            ],
        );
    }
}
