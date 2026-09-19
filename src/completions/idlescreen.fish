# fish completion for idlescreen(1) — maintained alongside src/cli.rs

set -l cmds status config enable disable timeout saver list inhibitors inhibit \
    start preview stop fps-overlay render-scale interactive restart logs doctor \
    clean completion bug-report self-update tui version about help

# Subcommand names + aliases
complete -c idlescreen -n __fish_use_subcommand -a "$cmds"
complete -c idlescreen -n __fish_use_subcommand -a "st cfg on off t sv ls inhib hold activate p x fps scale i rs log doc cl comp bug update upgrade ui v info"

# Global flags
complete -c idlescreen -s q -l quiet -d "suppress confirmations"
complete -c idlescreen -s h -l help -d "print help"
complete -c idlescreen -s V -l version -d "print version"

# config ops
complete -c idlescreen -n "__fish_seen_subcommand_from config cfg" -a "list get set path edit reset" -n "__fish_is_nth_token 3"
complete -c idlescreen -n "__fish_seen_subcommand_from config cfg" -s j -l json -d "JSON output"
complete -c idlescreen -n "__fish_seen_subcommand_from reset" -s y -l yes -d "skip confirmation"

# saver ops
complete -c idlescreen -n "__fish_seen_subcommand_from saver sv" -a "set list" -n "__fish_is_nth_token 3"
complete -c idlescreen -n "__fish_seen_subcommand_from saver sv" -s j -l json -d "JSON output"

# --json commands
for c in status st list ls inhibitors inhib timeout t version v
    complete -c idlescreen -n "__fish_seen_subcommand_from $c" -s j -l json -d "JSON output"
end
complete -c idlescreen -n "__fish_seen_subcommand_from version v" -s l -l long -d "extended info"

# doctor
complete -c idlescreen -n "__fish_seen_subcommand_from doctor doc" -s f -l fix -d "repair common issues"
complete -c idlescreen -n "__fish_seen_subcommand_from doctor doc" -s j -l json -d "JSON output"

# logs
complete -c idlescreen -n "__fish_seen_subcommand_from logs log" -s f -l follow -d "follow entries"
complete -c idlescreen -n "__fish_seen_subcommand_from logs log" -s n -l lines -d "entry count" -x

# clean
complete -c idlescreen -n "__fish_seen_subcommand_from clean cl" -s n -l dry-run -d "show what would be removed"

# self-update
complete -c idlescreen -n "__fish_seen_subcommand_from self-update update upgrade" -s c -l check -d "report status only"

# completion
complete -c idlescreen -n "__fish_seen_subcommand_from completion comp" -a "bash zsh fish nushell powershell elvish" -n "__fish_is_nth_token 3"

# inhibit
complete -c idlescreen -n "__fish_seen_subcommand_from inhibit hold" -s r -l reason -d "inhibitor reason" -x

# preview
complete -c idlescreen -n "__fish_seen_subcommand_from preview p" -s t -l timeout -d "auto-stop seconds" -x

# fps-overlay / render-scale
complete -c idlescreen -n "__fish_seen_subcommand_from fps-overlay fps" -a "on off status"
complete -c idlescreen -n "__fish_seen_subcommand_from render-scale scale" -a "default status"
