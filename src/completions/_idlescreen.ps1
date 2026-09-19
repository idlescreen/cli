# powershell completion for idlescreen(1) — maintained alongside src/cli.rs
# Install: idlescreen completion powershell | Out-String | Invoke-Expression

Register-ArgumentCompleter -Native -CommandName idlescreen -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $cmds = @(
        'status','config','enable','disable','timeout','saver','list','inhibitors',
        'inhibit','start','preview','stop','fps-overlay','render-scale','interactive',
        'restart','logs','doctor','clean','completion','bug-report','self-update',
        'tui','version','about','help',
        'st','cfg','on','off','t','sv','ls','inhib','hold','activate','p','x','fps',
        'scale','i','rs','log','doc','cl','comp','bug','update','upgrade','ui','v','info'
    )

    $words = $commandAst.CommandElements | ForEach-Object { $_.ToString() }
    $sub = if ($words.Count -ge 2) { $words[1] } else { $null }

    $candidates = @()
    if ($words.Count -le 2) {
        $candidates = $cmds
    } elseif ($sub -in 'config','cfg') {
        $candidates = if ($words.Count -eq 3) { @('list','get','set','path','edit','reset') } else { @('-j','--json','-y','--yes') }
    } elseif ($sub -in 'saver','sv') {
        $candidates = if ($words.Count -eq 3) { @('set','list') } else { @('-j','--json') }
    } elseif ($sub -in 'completion','comp') {
        $candidates = @('bash','zsh','fish','nushell','powershell','elvish')
    } elseif ($sub -in 'fps-overlay','fps') {
        $candidates = @('on','off','status')
    } elseif ($sub -in 'render-scale','scale') {
        $candidates = @('default','status')
    } elseif ($sub -in 'doctor','doc') {
        $candidates = @('-f','--fix','-j','--json')
    } elseif ($sub -in 'logs','log') {
        $candidates = @('-f','--follow','-n','--lines')
    } elseif ($sub -in 'clean','cl') {
        $candidates = @('-n','--dry-run')
    } elseif ($sub -in 'self-update','update','upgrade') {
        $candidates = @('-c','--check')
    } elseif ($sub -in 'inhibit','hold') {
        $candidates = @('-r','--reason')
    } elseif ($sub -in 'preview','p') {
        $candidates = @('-t','--timeout')
    } elseif ($sub -in 'status','st','list','ls','inhibitors','inhib','timeout','t') {
        $candidates = @('-j','--json')
    } elseif ($sub -in 'version','v') {
        $candidates = @('-j','--json','-l','--long')
    } else {
        $candidates = @('-q','--quiet','-h','--help')
    }

    $candidates |
        Where-Object { $_ -like "$wordToComplete*" } |
        ForEach-Object { [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_) }
}
