run LANG:
    docker build -t does-it-fail-{{LANG}} {{LANG}}
    docker run -it --rm --name does-it-fail-{{LANG}}-running does-it-fail-{{LANG}}