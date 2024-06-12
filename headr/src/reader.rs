use std::collections::VecDeque;
use std::io::BufRead;

/// Reads a specified number of lines from the input stream
pub fn read_lines(filename: &str, reader: &mut impl BufRead, num_lines: i64) -> Vec<String> {
    let mut lines = vec![];
    lines.push(filename_header(filename));

    let mut buf = String::new();
    if num_lines > 0 {
        for _ in 0..num_lines {
            // use read_line to preserve newline endings for both unix and windows
            let is_eof = reader
                .read_line(&mut buf)
                .expect("should read line from reader")
                == 0;
            if is_eof {
                break;
            }
            lines.push(buf.clone());
            buf.clear();
        }
    } else {
        let num_lines = -num_lines as usize;
        let mut ring_buffer = VecDeque::with_capacity(num_lines);
        // again, lines method not used because we want to preserve line endings for windows
        for _ in 0.. {
            let is_eof = reader
                .read_line(&mut buf)
                .expect("should read line from reader")
                == 0;
            if is_eof {
                break;
            }
            if ring_buffer.len() == num_lines {
                lines.push(ring_buffer.pop_front().expect("queue element should exist"));
            }
            ring_buffer.push_back(buf.clone());
            buf.clear();
        }
    }

    lines
}

/// Reads a given specified number of bytes from the input stream
pub fn read_bytes(
    filename: &str,
    reader: impl BufRead,
    num_bytes: i64,
) -> anyhow::Result<Vec<String>> {
    let mut res = vec![filename_header(filename)];
    let mut buf = vec![];

    if num_bytes > 0 {
        buf = reader
            .bytes()
            .take(num_bytes as usize)
            .collect::<Result<Vec<_>, _>>()?;
    } else {
        let num_bytes = -num_bytes as usize;
        let mut ring_buffer = VecDeque::with_capacity(num_bytes);

        for elem in reader.bytes() {
            if ring_buffer.len() == num_bytes {
                buf.push(
                    ring_buffer
                        .pop_front()
                        .expect("ring buffer element should exist"),
                );
            }
            ring_buffer.push_back(elem?);
        }
    }

    if buf.len() > 0 {
        res.push(format!("{}\n", String::from_utf8_lossy(&buf)));
    }

    Ok(res)
}

fn filename_header(filename: &str) -> String {
    format!("==> {filename} <==\n")
}
