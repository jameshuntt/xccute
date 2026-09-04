use std::ffi::{OsStr, OsString};
use std::marker::PhantomData;

use xccute::{
    CompositeArgvPart,
    CompositeShellCommand,
    CompositeShellRoot,
    RootableCompositeSurface,
};

// -----------------------------------------------------------------------------
// This section represents:
//
// #[derive(CompositeShell)]
// #[shell(program = "git")]
// pub struct Git {
//     #[composite(surface)]
//     remote: GitRemote,
// }
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default)]
struct Git;

impl CompositeShellRoot for Git {
    fn program() -> &'static OsStr {
        OsStr::new("git")
    }
}

impl Git {
    fn remote() -> <GitRemote as RootableCompositeSurface<Git>>::RootedSurface {
        GitRemote.rooted_surface()
    }
}

// -----------------------------------------------------------------------------
// This section represents:
//
// #[derive(CompositeSubCommand, Default)]
// #[subcommand(segment = "remote")]
// pub struct GitRemote {
//     #[composite(command)]
//     add: GitRemoteAdd,
// }
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
struct GitRemote;

#[derive(Debug, Clone)]
struct GitRemoteRooted<R>
where
    R: CompositeShellRoot,
{
    inner: GitRemote,
    _root: PhantomData<R>,
}

impl<R> RootableCompositeSurface<R> for GitRemote
where
    R: CompositeShellRoot,
{
    type RootedSurface = GitRemoteRooted<R>;

    fn rooted_surface(self) -> Self::RootedSurface {
        GitRemoteRooted {
            inner: self,
            _root: PhantomData,
        }
    }
}

impl CompositeArgvPart for GitRemote {
    fn push_argv_part(&self, out: &mut Vec<OsString>) {
        out.push(OsString::from("remote"));
    }
}

impl<R> GitRemoteRooted<R>
where
    R: CompositeShellRoot,
{
    fn add(
        self,
        name: impl Into<OsString>,
        url: impl Into<OsString>,
    ) -> GitRemoteAddRooted<R> {
        GitRemoteAddRooted {
            prefix: self.inner,
            inner: GitRemoteAdd::new(name, url),
            _root: PhantomData,
        }
    }
}

// -----------------------------------------------------------------------------
// This section represents:
//
// #[derive(SimpleSubCommand)]
// #[subcommand(segment = "add")]
// pub struct GitRemoteAdd {
//     #[builder(init_required, into)]
//     #[arg(positional)]
//     name: OsString,
//
//     #[builder(init_required, into)]
//     #[arg(positional)]
//     url: OsString,
// }
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct GitRemoteAdd {
    name: OsString,
    url: OsString,
}

impl GitRemoteAdd {
    fn new(name: impl Into<OsString>, url: impl Into<OsString>) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
        }
    }
}

impl CompositeArgvPart for GitRemoteAdd {
    fn push_argv_part(&self, out: &mut Vec<OsString>) {
        out.push(OsString::from("add"));
        out.push(self.name.clone());
        out.push(self.url.clone());
    }
}

#[derive(Debug, Clone)]
struct GitRemoteAddRooted<R>
where
    R: CompositeShellRoot,
{
    prefix: GitRemote,
    inner: GitRemoteAdd,
    _root: PhantomData<R>,
}

impl<R> CompositeShellCommand for GitRemoteAddRooted<R>
where
    R: CompositeShellRoot,
{
    fn program(&self) -> &OsStr {
        R::program()
    }

    fn push_args(&self, out: &mut Vec<OsString>) {
        self.prefix.push_argv_part(out);
        self.inner.push_argv_part(out);
    }
}

#[test]
fn git_remote_add_rooted_command_emits_argv() {
    let cmd = Git::remote().add("origin", "https://github.com/me/repo.git");

    assert_eq!(cmd.program(), OsStr::new("git"));

    assert_eq!(
        cmd.argv(),
        vec![
            OsString::from("remote"),
            OsString::from("add"),
            OsString::from("origin"),
            OsString::from("https://github.com/me/repo.git"),
        ]
    );

    assert_eq!(
        cmd.build_display(),
        "git remote add origin https://github.com/me/repo.git"
    );
}
