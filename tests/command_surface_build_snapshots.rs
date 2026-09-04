//! Command-surface hardening snapshots.
//!
//! This gate is intentionally broad and cheap: it proves every active handwritten
//! command builder participates in the common `ShellCommand` build path and has at
//! least one representative option-surface snapshot.

use xccute::ShellCommand;

fn assert_build<T: ShellCommand>(cmd: T, expected: &str) {
    assert_eq!(cmd.build(), expected);
}

fn assert_shell_impl<T: ShellCommand>() {}

#[test]
fn every_active_command_builder_uses_shell_command_trait() {
    // filesystem
    assert_shell_impl::<xccute::fs::CdBuilder>();
    assert_shell_impl::<xccute::fs::ChmodBuilder>();
    assert_shell_impl::<xccute::fs::ChownBuilder>();
    assert_shell_impl::<xccute::fs::CpBuilder>();
    assert_shell_impl::<xccute::fs::DfBuilder>();
    assert_shell_impl::<xccute::fs::DuBuilder>();
    assert_shell_impl::<xccute::fs::EchoBuilder>();
    assert_shell_impl::<xccute::fs::FileBuilder>();
    assert_shell_impl::<xccute::fs::LnBuilder>();
    assert_shell_impl::<xccute::fs::LinkBuilder>();
    assert_shell_impl::<xccute::fs::LsBuilder>();
    assert_shell_impl::<xccute::fs::MkdirBuilder>();
    assert_shell_impl::<xccute::fs::MvBuilder>();
    assert_shell_impl::<xccute::fs::NanoBuilder>();
    assert_shell_impl::<xccute::fs::RmBuilder>();
    assert_shell_impl::<xccute::fs::RmdirBuilder>();
    assert_shell_impl::<xccute::fs::StatBuilder>();
    assert_shell_impl::<xccute::fs::TouchBuilder>();

    // network/process/shell/user utility surfaces
    assert_shell_impl::<xccute::net::CurlBuilder>();
    assert_shell_impl::<xccute::net::FtpBuilder>();
    assert_shell_impl::<xccute::net::IpBuilder>();
    assert_shell_impl::<xccute::net::NetcatBuilder>();
    assert_shell_impl::<xccute::net::NetstatBuilder>();
    assert_shell_impl::<xccute::net::PingBuilder>();
    assert_shell_impl::<xccute::net::RsyncBuilder>();
    assert_shell_impl::<xccute::net::ScpBuilder>();
    assert_shell_impl::<xccute::net::SftpBuilder>();
    assert_shell_impl::<xccute::net::SsBuilder>();
    assert_shell_impl::<xccute::net::TelnetBuilder>();
    assert_shell_impl::<xccute::net::WgetBuilder>();
    assert_shell_impl::<xccute::proc::HtopBuilder>();
    assert_shell_impl::<xccute::proc::IdBuilder>();
    assert_shell_impl::<xccute::proc::KillBuilder>();
    assert_shell_impl::<xccute::proc::PgrepBuilder>();
    assert_shell_impl::<xccute::proc::PkillBuilder>();
    assert_shell_impl::<xccute::proc::PsBuilder>();
    assert_shell_impl::<xccute::proc::PsGrepBuilder>();
    assert_shell_impl::<xccute::proc::TopBuilder>();
    assert_shell_impl::<xccute::proc::UptimeBuilder>();
    assert_shell_impl::<xccute::proc::WBuilder>();
    assert_shell_impl::<xccute::proc::WhoBuilder>();
    assert_shell_impl::<xccute::shell::BashBuilder>();
    assert_shell_impl::<xccute::shell::ShBuilder>();
    assert_shell_impl::<xccute::shell::ZshBuilder>();
    assert_shell_impl::<xccute::user::AddUserBuilder>();
    assert_shell_impl::<xccute::user::GroupAddBuilder>();
    assert_shell_impl::<xccute::user::PasswdBuilder>();
    assert_shell_impl::<xccute::user::SuBuilder>();
    assert_shell_impl::<xccute::user::SudoBuilder>();
    assert_shell_impl::<xccute::user::UserAddBuilder>();
    assert_shell_impl::<xccute::user::UserModBuilder>();
    assert_shell_impl::<xccute::user::VisudoBuilder>();
    assert_shell_impl::<xccute::utilscript::AwkBuilder>();
    assert_shell_impl::<xccute::utilscript::CutBuilder>();
    assert_shell_impl::<xccute::utilscript::GrepBuilder>();
    assert_shell_impl::<xccute::grep::GrepBuilder>();
    assert_shell_impl::<xccute::utilscript::HeadBuilder>();
    assert_shell_impl::<xccute::utilscript::SedBuilder>();
    assert_shell_impl::<xccute::utilscript::SortBuilder>();
    assert_shell_impl::<xccute::utilscript::TailBuilder>();
    assert_shell_impl::<xccute::utilscript::TeeBuilder>();
    assert_shell_impl::<xccute::utilscript::UniqBuilder>();
    assert_shell_impl::<xccute::utilscript::WcBuilder>();
    assert_shell_impl::<xccute::utilscript::XargsBuilder>();
    assert_shell_impl::<xccute::third_party::GitBuilder>();
    assert_shell_impl::<xccute::CargoBuilder>();
    assert_shell_impl::<xccute::psql::PsqlCommandBuilder>();
}

#[test]
fn filesystem_builder_option_snapshots_are_stable() {
    assert_build(xccute::fs::CdBuilder::new("work").join("src"), "cd work/src");
    assert_build(xccute::fs::ChmodBuilder::new("755", "bin/tool").recursive(), "chmod -R 755 bin/tool");
    assert_build(
        xccute::fs::ChownBuilder::new("james", "/var/log/app.log")
            .group("staff")
            .recursive()
            .no_dereference()
            .from("root", Some("wheel".to_string())),
        "chown -R -h --from root:wheel james:staff /var/log/app.log",
    );
    assert_build(xccute::fs::CpBuilder::new("a", "b").recursive().force().verbose(), "cp -r -f -v a b");
    assert_build(xccute::fs::DfBuilder::new().human_readable().all().local().inodes().path("/"), "df -h -a -l -i /");
    assert_build(xccute::fs::DuBuilder::new("/var").summarize().human_readable().apparent_size().all().max_depth(2), "du -s -h --apparent-size -a --max-depth=2 /var");
    assert_build(xccute::fs::EchoBuilder::new("hello").quoted().no_newline().to_file("out.txt").append(), "echo -n \"hello\" >> out.txt");
    assert_build(xccute::fs::FileBuilder::new("archive.tar.gz").mime().brief().dereference().special_files(), "file --mime-type --brief --dereference --special-files archive.tar.gz");
    assert_build(xccute::fs::LnBuilder::new("src", "dst").symbolic().force().no_dereference().verbose(), "ln -s -f -n -v src dst");
    assert_build(xccute::fs::LsBuilder::new().long().all().human_readable().target("/tmp"), "ls -l -a -h /tmp");
    assert_build(xccute::fs::MkdirBuilder::new("target/demo").parents().verbose().mode("755"), "mkdir -p -v -m 755 target/demo");
    assert_build(xccute::fs::MvBuilder::new("old", "new").force().interactive().verbose().no_clobber().backup(), "mv -f -i -v -n -b old new");
    assert_build(xccute::fs::NanoBuilder::new("notes.md").sudo(), "sudo nano notes.md");
    assert_build(xccute::fs::RmBuilder::new("target/tmp").recursive().force().interactive().verbose(), "rm -r -f -i -v target/tmp");
    assert_build(xccute::fs::RmdirBuilder::new("empty-dir").recursive().verbose(), "rmdir -r -v empty-dir");
    assert_build(xccute::fs::StatBuilder::new("/tmp/file").follow_symlinks().format("%n:%s"), "stat -L --format %n:%s /tmp/file");
    assert_build(xccute::fs::TouchBuilder::new().create().no_create().date("2024-12-25").reference("ref").time("202412250101").file("target"), "touch -c -d 2024-12-25 -r ref -t 202412250101 target");
}

#[test]
fn network_process_shell_and_user_builder_snapshots_are_stable() {
    assert_build(xccute::net::CurlBuilder::new("https://example.invalid").silent().verbose().follow_redirects().output("out.json").header("Accept: application/json").method("POST").data("{x:1}").user("me:secret").insecure(), "curl -s -v -L -o out.json -H 'Accept: application/json' -X POST -d '{x:1}' -u me:secret -k https://example.invalid");
    assert_build(xccute::net::FtpBuilder::new("ftp.example.invalid").no_auto_login().script("commands.ftp").port(2121), "ftp -n -s: commands.ftp ftp.example.invalid:2121");
    assert_build(xccute::net::IpBuilder::new().command(xccute::net::IpCommand::Addr).args(&["show", "eth0"]), "ip addr show eth0");
    assert_build(xccute::net::NetcatBuilder::new().listen().udp().zero_io().input("in").output("out").host("127.0.0.1").port(8080), "nc -l -u -z -i in -o out 127.0.0.1 8080");
    assert_build(xccute::net::NetstatBuilder::new().tcp().udp().listening().programs().numeric().routes().all().interfaces(), "netstat -t -u -l -p -n -r -a -i");
    assert_build(xccute::net::PingBuilder::new("::1").ipv6().count(4).interval(1.5).timeout(2).packet_size(64).quiet(), "ping6 -c 4 -i 1.5 -W 2 -s 64 -q ::1");
    assert_build(xccute::net::RsyncBuilder::new("src/", "dst/").archive().verbose().recursive().compress().delete().dry_run().human_readable().progress().preserve_times().preserve_permissions(), "rsync -a -v -r -z --delete --dry-run -h --progress -t -p src/ dst/");
    assert_build(xccute::net::ScpBuilder::new("file", "host:/tmp/file").recursive().preserve_times().quiet().port(2222).identity_file("id_rsa"), "scp -r -p -q -P 2222 -i id_rsa file host:/tmp/file");
    assert_build(xccute::net::SftpBuilder::new("example.invalid").user("admin").identity_file("id_rsa").port(22).batch_file("batch.sftp").interactive(), "sftp -i id_rsa -P 22 -b batch.sftp -C admin@example.invalid");
    assert_build(xccute::net::SsBuilder::new().all().listening().tcp().udp().unix().process().state("ESTABLISHED").port(443), "ss -a -l -t -u -x -p state ESTABLISHED sport = :443");
    assert_build(xccute::net::TelnetBuilder::new("towel.blinkenlights.nl").timeout(5).escape_char('^').port(23), "telnet -t 5 -e ^ towel.blinkenlights.nl 23");
    assert_build(xccute::net::WgetBuilder::new("https://example.invalid/file").recursive().no_clobber().quiet().limit_rate("500k").user_agent("xccute-test").output_file("file"), "wget --recursive --no-clobber --quiet --limit-rate 500k --user-agent xccute-test -O file https://example.invalid/file");

    assert_build(xccute::proc::HtopBuilder::new().user("james").sort_key("PERCENT_CPU").tree().delay(10).no_colors().no_mouse(), "htop --user james --sort-key PERCENT_CPU --tree --delay 10 --no-color --no-mouse");
    assert_build(xccute::proc::IdBuilder::new().user_id_only().group_id_only().groups().name().username("james"), "id -u -g -G -n james");
    assert_build(xccute::proc::KillBuilder::new().signal("9").pid(1234), "kill -9 1234");
    assert_build(xccute::proc::PgrepBuilder::new().full().newest().oldest().exact().user("james").group("staff").pattern("node"), "pgrep -f -n -o -x -u james -G staff node");
    assert_build(xccute::proc::PkillBuilder::new("node").signal("TERM"), "pkill -TERM node");
    assert_build(xccute::proc::PsBuilder::new().all().user().format("pid,cmd").sort("-pid").arg("x"), "ps a u --format=pid,cmd --sort=-pid x");
    assert_build(xccute::proc::PsGrepBuilder::new("node"), "ps aux | grep node");
    assert_build(xccute::proc::TopBuilder::new().batch_mode().delay(1.5).iterations(5).user("james").pid(1234), "top -b -d 1.5 -n 5 -u james -p 1234");
    assert_build(xccute::proc::UptimeBuilder::new(), "uptime");
    assert_build(xccute::proc::WBuilder::new().short().user("james"), "w -s james");
    assert_build(xccute::proc::WhoBuilder::new().all().count().heading(), "who -a -q -H");

    assert_build(xccute::shell::BashBuilder::new().exit_on_error().verbose().login().command("echo hi"), "bash -e -x -l -c \"echo hi\"");
    assert_build(xccute::shell::ShBuilder::new().login().restricted().command("echo hi"), "sh -l -r -c \"echo hi\"");
    assert_build(xccute::shell::ZshBuilder::new().login().interactive().emacs().command("echo hi"), "zsh -l -i -e -c \"echo hi\"");

    assert_build(xccute::user::AddUserBuilder::new("agent").system().disabled_login().home("/srv/agent").shell("/usr/sbin/nologin").group("agents").gecos("Service Agent"), "adduser --system --disabled-login --home /srv/agent --shell /usr/sbin/nologin --ingroup agents --gecos \"Service Agent\" agent");
    assert_build(xccute::user::GroupAddBuilder::new("agents").system().gid(900), "groupadd --system -g 900 agents");
    assert_build(xccute::user::PasswdBuilder::new().stdin().user("james"), "echo \"<password>\" | passwd james");
    assert_build(xccute::user::SuBuilder::new().login().user("root").command("whoami"), "su -l root -c 'whoami'");
    assert_build(xccute::user::SudoBuilder::new("systemctl status app").preserve_env().login_shell().user("root").group("wheel"), "sudo -E -i -u root -g wheel systemctl status app");
    assert_build(xccute::user::UserAddBuilder::new("agent").system().create_home().home_dir("/srv/agent").shell("/usr/sbin/nologin").group("agents").uid(900), "useradd --system -m -d /srv/agent -s /usr/sbin/nologin -g agents -u 900 agent");
    assert_build(xccute::user::UserModBuilder::new("agent").rename("agent2").home_dir("/srv/agent2").move_home().shell("/bin/bash").group("agents").append_group("docker").uid(901).gid(902).lock(), "usermod -l agent2 -d /srv/agent2 -m -s /bin/bash -g agents -a -G docker -u 901 -g 902 -L agent");
    assert_build(xccute::user::VisudoBuilder::new().quiet().check_syntax().file("/etc/sudoers.d/app"), "visudo -q -c -f /etc/sudoers.d/app");
}

#[test]
fn util_third_party_database_builder_snapshots_are_stable() {
    assert_build(xccute::utilscript::AwkBuilder::new("{print $1}").field_separator(",").with_var("threshold", "10").file("input.csv"), "awk -F ',' threshold=10 '{print $1}' input.csv");
    assert_build(xccute::utilscript::CutBuilder::new().complement().only_delimited().delimiter(",").fields("1,3").bytes("1-10").characters("1-5").file("input.csv"), "cut --complement --only-delimited -d , -f 1,3 -b 1-10 -c 1-5 input.csv");
    assert_build(xccute::utilscript::GrepBuilder::new("NEEDLE").ignore_case().recursive().line_number().invert_match().word_regexp().extended_regex().file("a.log").file("b.log"), "grep -i -r -n -v -w -E NEEDLE a.log b.log");
    assert_build(xccute::grep::GrepBuilder::new("NEEDLE").file("a.log"), "grep NEEDLE a.log");
    assert_build(xccute::utilscript::HeadBuilder::new().lines(5).bytes(100).quiet().verbose().file("input.log"), "head -n 5 -c 100 -q -v input.log");
    assert_build(xccute::utilscript::SedBuilder::new().suppress_output().in_place().backup_extension(".bak").expression("s/a/b/g").file("input.txt"), "sed -n -i .bak -e s/a/b/g input.txt");
    assert_build(xccute::utilscript::SortBuilder::new().numeric().reverse().human_numeric().ignore_case().unique().key("2,2").file("input.txt"), "sort -n -r -h -f -u -k 2,2 input.txt");
    assert_build(xccute::utilscript::TailBuilder::new().lines(20).bytes(200).follow().quiet().file("input.log"), "tail -n 20 -c 200 -f -q input.log");
    assert_build(xccute::utilscript::TeeBuilder::new().append().ignore_interrupts().file("a.out").file("b.out"), "tee -a -i a.out b.out");
    assert_build(xccute::utilscript::UniqBuilder::new().count().repeated().unique_only().ignore_case().skip_fields(1).skip_chars(2).input("input.txt"), "uniq -c -d -u -i -f 1 -s 2 input.txt");
    assert_build(xccute::utilscript::WcBuilder::new().bytes().chars().lines().words().max_line_length().file("input.txt"), "wc -c -m -l -w -L input.txt");
    assert_build(xccute::utilscript::XargsBuilder::new().command("rm").max_args(2).delimiter("\\n").replace("{}").interactive().verbose(), "xargs rm -n 2 -d '\\n' -I {} -p --verbose");

    assert_build(xccute::third_party::GitBuilder::clone_mirror("https://example.invalid/repo.git", "repo.git"), "git clone --mirror https://example.invalid/repo.git repo.git");
    assert_build(xccute::CargoBuilder::new("test").package("xccute").manifest_path("Cargo.toml").features(&["a", "b"]).release().target("wasm32-unknown-unknown"), "cargo test --release --package xccute --manifest-path Cargo.toml --features a,b --target wasm32-unknown-unknown");
    assert_build(xccute::psql::PsqlCommandBuilder::new("SELECT 1;").database("app").user("postgres").quiet(), "psql -d app -U postgres -q -c \"SELECT 1;\"");
}
