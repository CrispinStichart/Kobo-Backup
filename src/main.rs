use filesize::file_real_size;
use std::path::Path;
use windows::UI::Notifications::{ToastNotification, ToastNotificationManager, ToastTemplateType};

const SOURCE: &str = r"C:\Users\Crispin Stichart\KoboReader.sqlite";
const DESTINATION: &str = r"F:\.kobo\KoboReader.sqlite";

fn copy_file_if_larger(source: &Path, dest: &Path) {
    // Check the size of the source file

    let source_size = file_real_size(source);
    let dest_size = file_real_size(dest);
    if source_size.is_err() {
        println!("Source didn't exist, and that's fine.");
        return;
    }
    let dest_size = if dest_size.is_err() {
        println!("Dest size was an error, so we're setting it to zero.");
        Ok(0)
    } else {
        dest_size
    };

    match (source_size, dest_size) {
        (Ok(s_size), Ok(d_size)) if s_size > d_size => {
            if std::fs::copy(source, dest).is_err() {
                show_error("Failed to copy the file.");
            } else {
                println!("Did the copy!");
            }
        }
        (Ok(s_size), Ok(d_size)) if s_size < d_size => {
            show_error("Error: KoboReader.sqlite file is smaller than the existing version.");
        }
        _ => {
            println!("Sizes are equal.")
        } // Sizes are equal or there was an error
    }
}

fn show_error(message: &str) {
    println!("{}", message);
    let appid = windows::core::HSTRING::from("Kobo Backup Script");
    let notifier = ToastNotificationManager::CreateToastNotifierWithId(&appid)
        .expect("Couldn't create the notifier???");
    let content = ToastNotificationManager::GetTemplateContent(ToastTemplateType::ToastText01);
    let content = content.expect("Couldn't unwrap content");
    let toast = ToastNotification::CreateToastNotification(&content).expect("lol");

    notifier.Show(&toast).expect("couldn't show message");
}

fn main() {
    // This should be your application logic
    let source_path = Path::new(DESTINATION);
    let dest_path = Path::new(SOURCE);

    copy_file_if_larger(source_path, dest_path);
}
