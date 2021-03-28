use headless_chrome::{Browser, Tab};
use std::sync::Arc;
use url::Url;

macro_rules! my_macro {
    (
        struct $name:ident {
            $($field_name:ident: $field_type:ty,)*
        }
    ) => {
        #[derive(Debug, Default)]
        struct $name {
            $($field_name: $field_type,)*
        }

        impl $name {
            fn get_field_names() -> Vec<&'static str> {
                vec![$(stringify!($field_name)),*]
                    .into_iter()
                    .map(|p| {
                        if p.starts_with("r#") { p.trim_start_matches("r#") }
                        else { p }
                    })
                    .collect()
            }
        }
    }
}

my_macro! {
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

fn visit_link(link: Url) -> Result<LinkPreview, failure::Error> {
    println!("visiting: {}", link.as_str());
    let mut link_preview_data = LinkPreview::default();
    let browser = Browser::default()?;

    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to(link.as_str())?;
    tab.wait_until_navigated()?;

    match parse_title(&tab) {
        Ok(title) => link_preview_data.title = title,
        Err(e) => println!("{}", e),
    };

    match parse_image(&tab) {
        Ok(img) => link_preview_data.image = img,
        Err(e) => println!("{}", e),
    }

    return Ok(link_preview_data);
}

fn parse_title(tab: &Arc<Tab>) -> Result<String, failure::Error> {
    tab.get_title()
}

fn parse_image(tab: &Arc<Tab>) -> Result<String, failure::Error> {
    parse_util(
        tab,
        String::from("meta[property=\"og:image\"]"),
        String::from("content"),
    )
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

fn main() {
    let open_graph_props = LinkPreview::get_field_names();
    for og_prop in open_graph_props {
        println!("{:#?}", og_prop);
    }

    match Url::parse("https://www.youtube.com/watch?v=09fNvoQMlGw") {
        Ok(link) => match visit_link(link) {
            Ok(data) => println!("open-graph data: {:#?}", data),
            Err(e) => println!("error parsing open graph data: {:?}", e),
        },
        Err(e) => println!("error parsing url: {:?}", e),
    }
}
