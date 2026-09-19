# elvish completion for idlescreen(1) — maintained alongside src/cli.rs
# Install: eval (idlescreen completion elvish | slurp)

var cmds = [status config enable disable timeout saver list inhibitors inhibit
    start preview stop fps-overlay render-scale interactive restart logs doctor
    clean completion bug-report self-update tui version about help]
var aliases = [st cfg on off t sv ls inhib hold activate p x fps scale i rs log
    doc cl comp bug update upgrade ui v info]
var global-flags = [-q --quiet -h --help -V --version]

set edit:completion:arg-completer[idlescreen] = {|@words|
  var n = (count $words)
  if (== $n 2) {
    put $@cmds $@aliases $@global-flags
  } elif (>= $n 3) {
    var cmd = $words[1]
    if (or (eq $cmd config) (eq $cmd cfg)) {
      if (== $n 3) { put list get set path edit reset } else { put -j --json -y --yes }
    } elif (or (eq $cmd saver) (eq $cmd sv)) {
      if (== $n 3) { put set list } else { put -j --json }
    } elif (or (eq $cmd completion) (eq $cmd comp)) {
      if (== $n 3) { put bash zsh fish nushell powershell elvish }
    } elif (or (eq $cmd fps-overlay) (eq $cmd fps)) {
      put on off status
    } elif (or (eq $cmd render-scale) (eq $cmd scale)) {
      put default status
    } elif (or (eq $cmd doctor) (eq $cmd doc)) {
      put -f --fix -j --json
    } elif (or (eq $cmd logs) (eq $cmd log)) {
      put -f --follow -n --lines
    } elif (or (eq $cmd clean) (eq $cmd cl)) {
      put -n --dry-run
    } elif (or (eq $cmd self-update) (eq $cmd update) (eq $cmd upgrade)) {
      put -c --check
    } elif (or (eq $cmd inhibit) (eq $cmd hold)) {
      put -r --reason
    } elif (or (eq $cmd preview) (eq $cmd p)) {
      put -t --timeout
    } elif (or (eq $cmd status) (eq $cmd st) (eq $cmd list) (eq $cmd ls) (eq $cmd inhibitors) (eq $cmd inhib) (eq $cmd timeout) (eq $cmd t)) {
      put -j --json
    } elif (or (eq $cmd version) (eq $cmd v)) {
      put -j --json -l --long
    } else {
      put $@global-flags
    }
  }
}
