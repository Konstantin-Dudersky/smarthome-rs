use dev-sync.nu 
use lib/run_in_ssh.nu
use dev-compile.nu

dev-compile
run_in_ssh "target-stop"
dev-sync
run_in_ssh "target-start"
