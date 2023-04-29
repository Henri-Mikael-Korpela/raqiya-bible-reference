use std::env;

use bible::TextId;

mod bible;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let Some(source_text_id_arg) = args.get(1) else {
        eprintln!("No source text ID given as a command line argument #1.");
        return;
    };

    let Some(source_text_id) = TextId::find_by_string(source_text_id_arg) else {
        eprintln!("No matching text ID given by given source text ID \"{}\".", source_text_id_arg);
        return;
    };

    let Some(destination_text_id_arg) = args.get(2) else {
        eprintln!("No destination text ID given as a command line argument #2.");
        return;
    };

    let Some(destination_text_id) = TextId::find_by_string(destination_text_id_arg) else {
        eprintln!("No matching text ID given by given destination text ID \"{}\".", destination_text_id_arg);
        return;
    };

    if source_text_id == destination_text_id {
        println!(
            "Source and destination text IDs are the same. Therefore no conversion is performed.",
        );
        return;
    }

    let Some(content) = args.get(3) else {
        eprintln!("No text content with references to convert given as a command line argument #3.");
        return;
    };

    let replaced_content =
        bible::replace_reference_matches_in(content.as_str(), &source_text_id, |reference| {
            reference.to_string(&destination_text_id)
        });

    println!("{}", replaced_content);
}
