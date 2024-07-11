use noodles::bam;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = std::env::args().collect();

    let align_path = &args[1].to_string();
    let out_path = &args[2].to_string();
    let start_ref = &args[3].to_string();

    let mut reader = bam::io::indexed_reader::Builder::default().build_from_path(&align_path)?;
    let header = reader.read_header()?;

    let refs: Vec<_> = header.reference_sequences().into_iter().map(|(refname, _)| refname).collect();

    let start_ref_idx = refs.iter().position(|&refname| refname == start_ref).unwrap();

    let end_ref_idx = if args.len() > 4 {
        let end_ref = &args[4].to_string();
        1 + refs.iter().position(|&refname| refname == end_ref).unwrap()
    }
    else {
        refs.len() - 1
    };

    let mut writer = bam::io::writer::Builder::default().build_from_path(&out_path)?;
    writer.write_header(&header)?;

    for rname in &refs[start_ref_idx..end_ref_idx] {
        println!("Copying records for {}", rname);

        let region = rname.to_string().parse()?;
        let query = reader.query(&header, &region)?;

        for result in query {
            let record = result?;
            writer.write_record(&header, &record)?;
        }
    }

    Ok(())
}
