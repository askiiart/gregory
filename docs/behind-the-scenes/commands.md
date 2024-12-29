# How commands are run

I was unable to find a way to directly run *multiple* commands via Docker/Podman. Instead of doing that, greg puts all the commands in a temporary script, mounts it inside, and then run it with 