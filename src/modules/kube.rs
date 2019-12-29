use std::marker::PhantomData;
use std::{process::Command};

use super::Module;
use crate::{powerline::Segment, terminal::Color, R, Error};
use std::string::FromUtf8Error;

pub struct Kube<S>(PhantomData<S>);

pub trait KubeScheme {
	const KUBE_FG: Color;
	const KUBE_BG: Color;
}

impl<S: KubeScheme> Kube<S> {
	pub fn new() -> Kube<S> {Kube(PhantomData)}
}

fn get_cluster_name(stdout: &Vec<u8>) -> Result<String, FromUtf8Error> {
	String::from_utf8(stdout.to_vec())
		.map(|raw| raw.trim().to_string())
		.map(|raw|raw.split("/").last().unwrap().to_string()
		)
}

impl<S: KubeScheme> Module for Kube<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
		let fg = S::KUBE_FG;
		let bg = S::KUBE_BG;
		let kube_cluster = Command::new("kubectl")
			.args(&["config", "current-context"])
			.output()
			.map_err(|e| Error::wrap(e, "kubectl command failed"));

		match kube_cluster {
			Ok(ref v) if v.status.success() => {
				let cluster_name = get_cluster_name(&v.stdout);
				match cluster_name {
					Ok(ref cn) => {
						segments.push(Segment::simple(&format!(" \u{2388} {} ", cn), fg, bg));
						Ok(())
					}
					_ => Ok(())
				}
			}
			_ => Ok(())
		}
	}
}
