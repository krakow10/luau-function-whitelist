use std::io::Read;

fn main()->Result<(),std::io::Error>{
    //convert cli args into file paths
    let args:Vec<std::path::PathBuf>=std::env::args().map(Into::into).collect();

    //first argument is the executable, slice args after
    for fname in &args[1..]{
        //read the file
        let mut file=std::fs::File::open(fname)?;
        let mut s=String::new();
        file.read_to_string(&mut s)?;

        //parse the string
        match full_moon::parse(s.as_str()){
            Ok(ast)=>{
                println!("ast {:?}", ast);
                // for (i,statement) in ast.nodes().stmts().enumerate(){
                //  println!("Statement #{}:\n{:?}",i,statement);
                // }
            },
            Err(e)=>{
                println!("parsing error: {e}");
            }
        }
    }
    Ok(())
}