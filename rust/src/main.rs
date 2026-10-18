use sha2::{Digest, Sha256};
use std::{env, fs, io, path::Path};
use tiny_http::{Response, Server};

// Hashes relative path + contents of every file; entries sorted by name so it's deterministic
// and matches the old Go WalkDir order.
fn walk(root: &Path, dir: &Path, h: &mut Sha256, files: &mut usize) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(dir)?.collect::<Result<_, _>>()?;
    entries.sort_by_key(|e| e.file_name());
    for e in entries {
        let p = e.path();
        if p.is_dir() {
            walk(root, &p, h, files)?;
        } else {
            h.update(p.strip_prefix(root).unwrap().to_string_lossy().as_bytes());
            h.update([0u8]);
            io::copy(&mut fs::File::open(&p)?, h)?;
            *files += 1;
        }
    }
    Ok(())
}

fn checksum(dir: &str) -> io::Result<(String, usize)> {
    let (mut h, mut n) = (Sha256::new(), 0);
    walk(Path::new(dir), Path::new(dir), &mut h, &mut n)?;
    Ok((h.finalize().iter().map(|b| format!("{b:02x}")).collect(), n))
}

fn main() {
    let dir = env::var("CHECKSUM_DIR").unwrap_or_else(|_| "/data".into());
    let server = Server::http("0.0.0.0:4000").unwrap();
    for req in server.incoming_requests() {
        let res = match req.url().split('?').next().unwrap() {
            "/health" => Response::from_string("ok"),
            "/checksum" => match checksum(&dir) {
                Ok((sum, n)) => Response::from_string(format!(
                    r#"{{"dir":"{dir}","algo":"sha256","files":{n},"checksum":"{sum}"}}"#
                )),
                Err(e) => Response::from_string(e.to_string()).with_status_code(500),
            },
            _ => Response::from_string("not found").with_status_code(404),
        };
        let _ = req.respond(res);
    }
}
