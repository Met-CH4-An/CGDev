# CGDev

# Настройка пользователя (один раз)
git config --global user.name "Твоё Имя"
git config --global user.email "email@пример.com"

# Инициализация репозитория в папке
git init

# Проверить статус файлов
git status

# Добавить файлы в индекс (готовы к коммиту)
git add <файл>       # Один файл
git add .            # Все изменения в текущей папке

# Создать коммит с сообщением
git commit -m "Сообщение"

# Просмотр веток
git branch

# Создать новую ветку
git branch <ветка>

# Переключиться на ветку
git checkout <ветка>

# Создать и сразу перейти на ветку
git checkout -b <ветка>

# Удалить ветку
git branch -d <ветка>    # Если ветка слита
git branch -D <ветка>    # Принудительно удалить

# Добавить удалённый репозиторий (GitHub)
git remote add origin <URL>

# Отправить изменения на GitHub
git push -u origin master    # Первый пуш
git push                     # Последующие пуши

# Получить изменения с GitHub
git pull

# Отменить локальные изменения в файле
git checkout -- <файл>