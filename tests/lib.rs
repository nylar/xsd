use std::collections::HashMap;
use std::default::Default;

const WORK_DIR: &'static str = env!("CARGO_MANIFEST_DIR");

#[test]
fn parse_xsd1() {
    let path = format!("{}/tests/testdata/xsd1.xml", WORK_DIR);

    let parser = xsd::Parser::new(&path).unwrap();

    let mut namespaces = HashMap::new();
    namespaces.insert(
        "xsd".to_owned(),
        "http://www.w3.org/2001/XMLSchema".to_owned(),
    );
    namespaces.insert("mns".to_owned(), "http://example.org/order.xsd".to_owned());

    let expected = vec![
        xsd::Elements::Schema(xsd::schema::Schema {
            element_form_default: Some(xsd::shared::FormDefault::Qualified),
            namespaces: namespaces,
            target_namespace: Some("http://example.org/order.xsd".to_owned()),
            ..Default::default()
        }),
        xsd::Elements::Element(xsd::element::Element {
            name: Some("Order".to_owned()),
            r#type: Some("mns:OrderType".to_owned()),
            ..Default::default()
        }),
        xsd::Elements::ComplexType(xsd::complex_type::ComplexType {
            name: Some("OrderType".to_owned()),
            sequence: Some(xsd::complex_type::Sequence {
                elements: vec![
                    xsd::element::Element {
                        name: Some("ShippingAddress".to_owned()),
                        r#type: Some("mns:Address".to_owned()),
                        max_occurrences: Some(xsd::shared::Occurrence::Limit(2)),
                        ..Default::default()
                    },
                    xsd::element::Element {
                        name: Some("BillingAddress".to_owned()),
                        r#type: Some("mns:Address".to_owned()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            attribute: Some(xsd::complex_type::Attribute {
                name: Some("Date".to_owned()),
                r#type: Some("xsd:date".to_owned()),
                ..Default::default()
            }),
            ..Default::default()
        }),
        xsd::Elements::ComplexType(xsd::complex_type::ComplexType {
            name: Some("Address".to_owned()),
            sequence: Some(xsd::complex_type::Sequence {
                elements: vec![
                    xsd::element::Element {
                        name: Some("name".to_owned()),
                        r#type: Some("xsd:string".to_owned()),
                        ..Default::default()
                    },
                    xsd::element::Element {
                        name: Some("street".to_owned()),
                        r#type: Some("xsd:string".to_owned()),
                        ..Default::default()
                    },
                    xsd::element::Element {
                        name: Some("city".to_owned()),
                        r#type: Some("xsd:string".to_owned()),
                        ..Default::default()
                    },
                    xsd::element::Element {
                        name: Some("county".to_owned()),
                        r#type: Some("xsd:string".to_owned()),
                        ..Default::default()
                    },
                    xsd::element::Element {
                        name: Some("postcode".to_owned()),
                        r#type: Some("xsd:string".to_owned()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
            attribute: Some(xsd::complex_type::Attribute {
                name: Some("country".to_owned()),
                r#type: Some("xsd:NMTOKEN".to_owned()),
                ..Default::default()
            }),
            ..Default::default()
        }),
    ];

    assert_eq!(parser.elements, expected);
}
