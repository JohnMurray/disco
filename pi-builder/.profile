# Support different ways of getting the directory containing this .profile for different
# shells (zsh and bash are most common... probably)
case $SHELL in
    */zsh)
        DIR=${0:a:h}
        export PATH="${DIR}:$PATH"
        ;;
    */bash)
        DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
        export PATH="${DIR}:$PATH"
        ;;
    *)
        echo "Unsupported shell: $SHELL"
        echo "Currently only 'zsh' and 'bash' are supported"
        exit 1
        ;;
esac