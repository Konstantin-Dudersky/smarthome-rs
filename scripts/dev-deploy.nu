use dev-sync.nu 
use lib/run_in_ssh.nu
use dev-compile.nu

dev-compile
ssh -t pi@target "sudo systemctl stop smarthome"
dev-sync
ssh -t pi@target "sudo systemctl start smarthome"
