FROM denoland/deno:ubuntu

WORKDIR /app

USER deno
COPY . .

# Entrypoint is set to "deno", this is why you don't need to specify it in the command.
CMD ["run", "--allow-read", "main.ts"]