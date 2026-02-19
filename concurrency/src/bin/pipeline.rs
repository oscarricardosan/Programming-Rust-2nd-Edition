use std::{fs::{self, OpenOptions}, io::{self, Error, Write}, path::{Path, PathBuf}, sync::mpsc, thread};
use sha2::{Sha256, Digest};


#[derive(Debug)]
struct  Document {
    index: usize,
    hash: String,
    path: PathBuf,
}



fn main(){
    let documents: Vec<PathBuf> = Path::new("documents")
        .read_dir()
        .unwrap()
        .map(|path| path.unwrap().path())
        .collect();


    let (sender_move_files, handle_move_files)= move_file();
    let (sender_error, handle_errors)= process_errors();
    let (sender_successful, handle_successful)= process_successful(sender_move_files.clone());
    let (receiver_indexed_file, handle_file_indexer)= index_files(documents);
    let handle_check_files= check_files(receiver_indexed_file, sender_error.clone(), sender_successful.clone());


    handle_file_indexer.join().unwrap().unwrap();
    handle_check_files.join().unwrap();
    
    drop(sender_error);
    drop(sender_successful);
    drop(sender_move_files);    


    let documents= handle_move_files.join().unwrap();

    handle_errors.join().unwrap();
    handle_successful.join().unwrap();

    println!("Archivos Procesados: ");
    
    dbg!(documents);


}

fn index_files(documents: Vec<PathBuf>) -> (mpsc::Receiver<Document>, thread::JoinHandle<io::Result<()>>)
{
    let (sender, receiver)= mpsc::channel();

    let handle= thread::spawn(move || {
        for (index, filename ) in documents.iter().enumerate() {
            let hash= {
                let text_bytes= fs::read(filename)?;
                let mut hasher = Sha256::new();
                hasher.update(text_bytes);
                hex::encode(hasher.finalize())
            };
            
            let document= Document{
                index,
                hash,
                path: filename.to_owned()
            };
            if sender.send(document).is_err(){
                break;
            }
        }
        Ok(())
    });

    (receiver, handle)

}


fn check_files(
    receiver_document: mpsc::Receiver<Document>, handle_error: mpsc::Sender<Error>, handle_successful: mpsc::Sender<Document>
)-> thread::JoinHandle<()>{
    thread::spawn(move || {
        for document in receiver_document {
            if document.hash.ends_with('5'){
                let error = io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{}) Hash '{}' terminado en 5 no valido", document.index, document.hash),
                );
                if handle_error.send(error).is_err(){
                    break;
                }
            }else{
                if handle_successful.send(document).is_err(){
                    break;
                }
            }
        }
    })
}

fn process_errors()-> (mpsc::Sender<Error>, thread::JoinHandle<()>)
{
    let (sender, receiver)= mpsc::channel();

    let handler= thread::spawn(move|| {
        if let Err(e) = fs::remove_file("errors.txt") {
            if e.kind() != io::ErrorKind::NotFound {
                panic!("{}", e);
            }
        }
        
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("errors.txt").unwrap();

        for error in receiver{
            println!(" ERROR: {}", error);
            file.write_all(format!("{}\n", error).as_bytes()).unwrap();
        }
    });

    (sender, handler)
}

fn process_successful(handle_file: mpsc::Sender<Document>)-> (mpsc::Sender<Document>, thread::JoinHandle<()>)
{
    let (
        sender, 
        receiver
    ): (mpsc::Sender<Document>, mpsc::Receiver<Document>) = mpsc::channel();

    let handler= thread::spawn(move|| {
        if let Err(e) = fs::remove_file("successful.txt") {
            if e.kind() != io::ErrorKind::NotFound {
                panic!("{}", e);
            }
        }
        
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("successful.txt").unwrap();

        for document in receiver{
            file.write_all(format!("{},{}\n", document.hash, document.path.to_string_lossy()).as_bytes()).unwrap();
            if handle_file.send(document).is_err(){
                break;
            }
        }
    });
    
    (sender, handler)
}


fn move_file()-> (mpsc::Sender<Document>, thread::JoinHandle<Vec<Document>>)
{
    let (
        sender, 
        receiver
    ): (mpsc::Sender<Document>, mpsc::Receiver<Document>) = mpsc::channel();

    if let Err(e) = fs::remove_dir_all("documents_processed") {
        if e.kind() != io::ErrorKind::NotFound {
            panic!("{}", e);
        }
    }
    fs::create_dir("documents_processed").unwrap();

    let handler= thread::spawn(move|| {
        let mut documents:Vec<Document>= vec![];
        for document in receiver{
            fs::copy(
                &document.path, 
                format!("documents_processed/{}", &document.path.file_name().unwrap().to_string_lossy())
            ).unwrap();
            documents.push(document);
        }

        documents
    });
    
    (sender, handler)
}