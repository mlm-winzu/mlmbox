# Сервис: ConfirmationService

## 1. Описание

**`ConfirmationService`** — это клиентский сервис, который предоставляет пользователю полный набор инструментов для управления сессиями подтверждения (`Confirmation`). Он позволяет запрашивать информацию о формах, которые необходимо подтвердить, и выполнять все необходимые действия (вводить коды, отменять, запрашивать новые коды).

Важно отметить, что этот сервис **не позволяет создавать** сессии подтверждения. Они генерируются на бэкенде автоматически как реакция на другие, более важные действия пользователя (например, запрос на вывод средств).

## 2. Общий сценарий использования (Workflow)

Типичный процесс подтверждения действия выглядит так:

1.  Пользователь инициирует важное действие (например, нажимает "Вывести средства").
2.  Бэкенд создает `Confirmation` и возвращает ее `id`.
3.  Клиентское приложение делает запрос `ConfirmationService.Get`, чтобы получить полную структуру формы и понять, какие поля нужно отобразить пользователю.
4.  Если требуется поле с кодом (например, `EMAIL_CODE`), а у пользователя его нет, он нажимает "Отправить код". Клиент вызывает `ConfirmationService.GenerateCode`.
5.  Пользователь заполняет все необходимые поля (например, вводит пароль и код из email).
6.  Клиент собирает все введенные данные и отправляет их с помощью `ConfirmationService.Confirm`.
7.  Сервер проверяет данные и возвращает `ConfirmResponse` с детальным результатом по каждому полю.
8.  Если все верно, `Confirmation.status` меняется на `CONFIRMED`, и бэкенд выполняет изначально запрошенное действие (вывод средств).

## 3. Описание методов (RPC)

### `rpc Get (biconom.types.Confirmation.Id) returns (biconom.types.Confirmation)`
- **Назначение**: Получить полную информацию о конкретной форме подтверждения по ее `id`.
- **Использование**: Это первый шаг после того, как бэкенд уведомил клиента о необходимости подтверждения. Ответ содержит `available_fields` и `verification_groups`, на основе которых клиент строит UI для пользователя.

### `rpc List (ListRequest) returns (biconom.types.Confirmation.List)`
- **Назначение**: Получить список форм подтверждения для **текущего аутентифицированного пользователя**.
- **Использование**: Позволяет отобразить пользователю все его активные сессии подтверждения, если он, например, закрыл вкладку и хочет вернуться к процессу.
- **Параметры `ListRequest`**:
  - `optional status`: Позволяет отфильтровать формы по статусу (например, показать только `PENDING`).
  - `optional cursor`: **Курсор** для пагинации. В него передается `biconom.types.Confirmation.Id` последней полученной записи, чтобы получить следующую "страницу".
  - `optional sort`: **Необязательное** поле. Если оно не указано, бэкенд использует безопасные значения по умолчанию (например, `direction: BACKWARD`, `limit: 50`), чтобы вернуть последние активные формы.

### `rpc Confirm (biconom.types.Confirmation.ConfirmRequest) returns (biconom.types.Confirmation.ConfirmResponse)`
- **Назначение**: Отправить введенные пользователем данные на проверку.
- **Использование**: Это ключевой метод, который пользователь вызывает для завершения верификации.
- **Ответ `ConfirmResponse`**: Возвращает не только общий статус (`APPROVED`/`REJECTED`), но и детальный результат по каждому полю, что позволяет клиенту подсветить конкретные ошибки (например, "Неверный пароль" или "Срок действия кода истек").

### `rpc Cancel (biconom.types.Confirmation.Id) returns (biconom.types.Confirmation)`
- **Назначение**: Отменить активную сессию подтверждения.
- **Использование**: Позволяет пользователю безопасно прервать операцию. Статус `Confirmation` меняется на `CANCELLED`.

### `rpc GenerateCode (biconom.types.Confirmation.GenerateCodeRequest) returns (biconom.types.Confirmation)`
- **Назначение**: Запросить генерацию нового одноразового кода для конкретного поля.
- **Использование**: Клиент вызывает этот метод, когда пользователь нажимает кнопку "Отправить код еще раз".
- **Ограничения**: Количество вызовов этого метода для одной формы ограничено полем `code_generation_attempt_limit` в `ConfirmationPolicy`.
