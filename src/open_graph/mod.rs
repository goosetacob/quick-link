use headless_chrome::{Browser, Tab};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use url::Url;

macro_rules! spice_up {
    (
        struct $name:ident {
            $($field_name:ident: $field_type:ty,)*
        }
    ) => {
        #[derive(Debug, Default, Serialize)]
        pub struct $name {
            $( pub $field_name: $field_type,)*
        }

        impl $name {
            fn get_field_names() -> Vec<&'static str> {
                vec![$(stringify!($field_name)),*]
                    .into_iter()
                    .map(|p| {
                        if p.starts_with("r#") { p.trim_start_matches("r#")}
                        else { p }
                    })
                    .collect()
            }

            fn from_hashmap(hm: HashMap<&str, String>) -> $name {
                let mut opd = $name::default();
                for (&k, v) in hm.iter() {
                  match k {
                    $(stringify!($field_name) => { opd.$field_name = v.clone() }, )*
                    "type" => { opd.r#type = v.clone() }, // TODO: edge case
                    _ => { println!("unsupported open graph porperty: {}", k) }
                  }
                }
                return opd;
            }
        }
    }
}

spice_up! {
    struct LinkPreview {
        title: String,
        r#type: String,
        image: String,
        url: String,
        audio: String,
        description: String,
        determiner: String,
        locale: String,
        locale_alternate: String,
        site_name: String,
        video: String,
    }
}

pub fn collect_data(link: &Url) -> Result<LinkPreview, failure::Error> {
    let open_graph_props = LinkPreview::get_field_names();
    let mut link_preview_data = HashMap::new();
    let browser = Browser::default()?;

    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to(link.as_str())?;
    tab.wait_until_navigated()?;

    for ogp_key in open_graph_props {
        match parse_util(
            &tab,
            format!("meta[property=\"og:{}\"]", ogp_key),
            format!("content"),
        ) {
            Err(e) => {
                println!("no value for {}: {}", ogp_key, e);
                continue;
            }
            Ok(ogp_value) => link_preview_data.insert(ogp_key, ogp_value),
        };
    }

    return Ok(LinkPreview::from_hashmap(link_preview_data));
}

fn parse_util(tab: &Arc<Tab>, selector: String, prop: String) -> Result<String, failure::Error> {
    let element = match tab.find_element(&selector) {
        Err(_) => return Err(failure::err_msg("failed to find element")),
        Ok(el) => el,
    };

    let attributes = match element.get_attributes() {
        Err(_) => return Err(failure::err_msg("failed to find attributes")),
        Ok(attributes) => match attributes {
            Some(attrs) => attrs,
            None => return Err(failure::err_msg("element has no attributes")),
        },
    };

    let value = match &mut attributes.get(&prop) {
        None => return Err(failure::err_msg("no value")),
        Some(content) => content.clone(),
    };

    Ok(value)
}
