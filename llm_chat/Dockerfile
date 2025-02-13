# Базовый образ с Rust и необходимые зависимости
FROM rust:latest

# Устанавливаем рабочую директорию
WORKDIR /app

# Копируем файлы проекта
COPY . .

# Сборка бинарника
RUN cargo build --release

# Запускаем бинарник
CMD ["./target/release/service"]
