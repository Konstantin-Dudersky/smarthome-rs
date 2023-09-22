use lib/sync.nu 
use lib/run_in_ssh.nu

run_in_ssh "target-stop"
sync
run_in_ssh "target-start"
