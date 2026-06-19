# Предисловие

Данная статья создана больше для меня самого и для моих товарищей по команде, которым также предстоит поработать с перечисленными здесь инструментами, поэтому я в этой статье не претендую на истину о том, как правильно проектировать приложение, как писать код и как строить архитектуру. Однако мне кажется, что для многих материал всё равно может оказаться полезным, поэтому решил выложить эту работу, тем более что я не смог обнаружить каких-либо иных источников на русском языке про GPUI, да и на английском их также немного. Я открыт к отзывам, советам, критике, буду рад прочитать ваши комментарии, если будут хорошие советы, то постараюсь обязательно исправить. Приятного чтения!

# Введение

GPUI является быстрым и производительным UI фреймворком на Rust c GPU ускорением от команды разработчиков, создавших редактор кода Zed, который как раз его использует для UI части. Данный фреймворк вышел относительно недавно, поэтому его API немного нестабильный, во многих источниках, на которые Вы могли наткнуться на просторах Интернета, включая примеры в официальном репозитории, написаны на более старой версии, из-за чего изучать данный фреймворк становится ещё сложнее. Он поддерживает такие платформы как macOS, Windows, Linux и FreeBSD. В данной работе применяется версия `0.2.2`.

Для изучения GPUI я решил создать небольшой pet-проект. Он из себя представляет попросту информационную доску о стендах из ДжоДжо: имя, фото и лепестковая диаграмма с характеристика, которые представлялись в аниме. Также в проекте будут рассмотрены фичи по выбору темы и языка интерфейса.

Ниже будут представлены материалы по самой GPUI, которые мне показались полезными: <br>
[Мини туториал](https://github.com/hedge-ops/gpui-tutorial) <br>
[Официальный репозиторий](https://github.com/zed-industries/zed/tree/main/crates/gpui) <br>
[Официальная документация](https://docs.rs/gpui/latest/gpui/index.html) <br>
[Сборник разных материалов по GPUI, включая документации, библиотеки, проекты](https://github.com/zed-industries/awesome-gpui/blob/main/README.md)

# Архитектура проекта

Для проекта была выбрана луковичная архитектура, структура вышла примерно следующей:
```
├── Cargo.toml                # Конфигурация Workspace
├── assets/                   # Ассеты приложения
│   ├── data/                 # Базы данных
│   ├── icons/                # Иконки
│   ├── images/               # Изображения
│   └── locales/              # Файлы локализации
└── crates/
    ├── app/                  # Точка входа, инициализация GPUI
    │   └── src/main.rs
    ├── core/                 # Ядро проекта
    │   └── src/
    │       ├── types/        # Общие типы и enum'ы
    │       ├── models/       # Чистые модели и бизнес-логика
    │       ├── repositories/ # Трейты для работы с данными
    │       ├── services/     # Сервисы для общения с данными
    │       └── lib.rs
    ├── infrastructure/       # Реализация интерфейсов и реализация элементов инфраструктуры
    │   └── src/
    │       ├── dtos/         # Трансферные объекты данных
    │       ├── mappers/      # Переводы между DTO и моделями
    │       ├── repositories/ # Конкретная реализация репозиториев
    │       ├── file/         # Структуры для работы с локальными файлами
    │       └── lib.rs
    ├── di/                   # Внедрение зависимостей 
    │   └── src/
    │       ├── dependency_injector.rs
    │       └── lib.rs
    └── ui/                   # UI слой
        └── src/
            ├── shared/   # Общие UI-компоненты
            ├── features/     # Большие отдельные фичи, экраны, окна
            ├── locale.rs     # Менеджер локализации
            ├── theme.rs      # Конфигурация тем оформления
            └── lib.rs
```

Луковичная архитектура является одним из видов многослойной архитектуры, к которому также относятся чистая и гексагональная. Принцип построения и концепции в них во всех схожи, отличаются лишь реализации этих правил. 

Архитектура как уже стала ясно состоит из слоёв, где каждый слой зависит от слоёв внутри него, однако ничего не знает о слое, внешнем для него. В Интернете Вы наверняка сможете найти материалы более подробные и точные, чем в данной статье, поэтому я пройдусь лишь вскользь.

## Ядро

В ядре представлена основная бизнес-логика и реализация этой логики, которая не зависит от UI фреймворка. Ядро является главным внутренним слоем, вокруг которого строятся внешние слои. Разберём, из чего он состоит.

### Модели

Модель (или же сущность) содержит в себе лишь поля информации, которую она несёт, и методы для работы с этими данными. Данный объект ничего не знает о том, как он будет применятся, не знает ничего о внешнем слое, даже о том, откуда берутся данные для его создания. Для большего удобства создаётся также отдельно директория `types`, которая хранит в себе типы и перечисления, призванные представить информацию в модели в более удобной форме.

### Репозитории

Репозиторий в ядре является обычным трейтом (интерфейсом), который указывает, какие манипуляции с моделями мы можем совершать, как их доставать и тому подобное.

### Сервисы

Сервисы в ядре являются последним слоем, именно с ними UI взаимодействует для получения нужных данных, и как уже стало понятно, сервис, общаясь с репозиторием, реализовывает методы общения внешних слоёв с ядром.

## Инфраструктура

Данный слой включает в себя те компоненты проекта, которые предназначены для извлечения и хранения данных, включая имплементации репозиториев.

### DTO

DTO (Data Transfer Object) $-$ структуры данных, которые несут в себе лишь информацию из источника данных и служат для транспортировки этой информации из внешних источников. Они зачастую обладают методами сериализации и десериализации.

### Мапперы

Маппер является компонентом, выполняющим лишь 2 задачи $-$ преобразование DTO в модель и обратно.

### Репозитории

Именно здесь создаётся конкретная реализация трейта репозитория из ядра, которая привязана к конкретному источнику данных. Для одного трейта репозитории может быть несколько имплементаций репозиториев в инфраструктуре, если они относятся к разным базам данных.

### Остальные компоненты

В инфраструктуре реализовываются компоненты, отвечающие за обращение к источникам данных (БД, файлы, API и тд.), реализацию клиентов внешних сервисов (Gateway, API клиент и тд.), реализацию очередей и прочего рода адаптеров. В конкретно моём проекте имеется лишь один такой слой `file`, который содержит компоненты для работы с локальными файлами.

## DI

DI (Dependency Injection) нужен для внедрения зависимостей извне. Во многих языках и платформах DI обычно имеет более сложные и широкие возможности для управления зависимостями в проекте, однако в моём проекте он значительно проще, так как реализовывает лишь создание глобальных объектов и предоставляет доступ к ним.

## UI

Последним и основным для проекта слоем является пользовательский интерфейс, который и использует UI фреймворк для создания самого приложения. Во многих проектах для слоя презентации используется один из паттерном семейства MV* (MVC, MVP, MVVM), однако для GPUI используется немного иной подход. Мне неизвестно, есть ли у этого подхода какое-то название, поэтому дал ему свой название: `State-Entity-View`. Чуть позже объясню её суть, сейчас разберём основную структуру, которую я применил для проект.

### Тема

В `theme.rs` определяется структура для управления темой проекта. Данная структура должна управлять не только основными свойствами UI, которые должны регулярно применяться, такие как цвет, отступы и тд, но и при наличии нескольких тем, предоставлять механизм для их смены.

### Локализация

В `locale.rs` определяется структура для работы с языковыми пакетами, в моём случае были использованы `ftl` (Fluent Translation List). Она хранит в себе выбранный язык и извлекает нужные переводы для передачи в UI.

### Компоненты

В директории `shared` должны содержаться те UI компоненты, которые будут повторно применены в разных компонентах, эти компоненты будем называть ___общими___.

### Фичи

Самым сложным по структуре является директория `features`. Я для себя определил ряд правил по её построению, которые мне показались более удобными для ориентации в крупном проекте, однако сильно сложными и излишними для маленького проекта.

На первом слое директории `features` определены компоненты, которые представляют из себя отдельный экран, окно или которые реализуют сложную динамическую логику изменения, зависящее от действий пользователя. Эти компоненты будем называть ___фичами___. Для каждой из этих ___фич___ строится директория следующего вида:
```
specific_feature/             # Конкретная фича
├── components/               # Внутренние компоненты фичи
├── state.rs                  # Логика управления состоянием
├── view.rs                   # Интерфейс фичи
└── mod.rs
```

#### Состояние

В данном слое определяются те поля, которые используются в UI для отрисовки, а также логика для манипуляций с ними.

#### Вид

Здесь определяется структура, у которой определены поля типов `gpui::Entity<SpecificFeatureState>` и `Subscription` и которая имплементирует трейт `Render`.

Тип `gpui::Entity<T>` представляет из себя сильный, типизированный указатель на структуру, которая управляется GPUI для отправки и принятия уведомлений. Тип `Subscription`, как ясно из названия, является дескриптором подписки, управляемая GPUI. Трейт `Render` определяет метод `render`, который рисует view в дереве элементов.

#### Внутренние компоненты

В директории `specific_feature/components` расположились те компоненты, которые применяются исключительно в `specific_feature`, эти компоненты будем называть ___внутренними___. Для ___внутренних компонент___ определяется следующая структура:
```
components/
├── multiple_component_1/
├── multiple_component_2/
├── ...
├── single_component_1.rs
├── single_component_2.rs
├── ...
└── mod.rs
```

Здесь `single_component_*.rs` реализовывают структуры, для которых которых определяется макрос `IntoElement`. Для таких структур должен быть определён трейт `RenderOnce`, который определяется для отрисовки недолгоживущих компонент.

Директории `multiple_component_*` определяются для тех ___внутренних компонент___, которые сами состоят включают себя ещё слой ___внутренних компонент___. Структура этих директорий следующая:
```
multiple_component_*/
├── components/
├── multiple_component_*.rs
└── mod.rs
```

Здесь `multiple_component_*.rs` подобно `single_component_*.rs` определяет `IntoElement` и реализовывает ___внутренний компонент___ с тем лишь отличием, что для него самого определены свои ___внутренние компоненты 2-го порядка вложенности___. Для директории `components` здесь структура определяется аналогичным образом, что и ранее, и так до $n$-ого порядка вложенности.

Все эти правила для построения структуры ___внутренних компонентов___ также актуальны для построения ___общих компонентов___.

## Приложение

Наконец всё это объединяется в `app/src/main.rs`, где в функции `main` происходит сборка и запуск приложения.

# Создание проекта

В корневой директории у нас имеется следующая структура файлов:
```
.
├── assets/
├── crates/
├── Cargo.lock
└── Cargo.toml
```

В Cargo.toml заполняем разделы `workspace` и `workspace.dependencies`. В разделе `workspace` обязательно указываем список крейтов и крейт для запуска:
```toml
[workspace]
members = ["crates/app", "crates/core", "crates/di", "crates/infrastructure", "crates/ui"]
default-members = ["crates/app"]
resolver = "2"
```
В разделе `workspace.dependencies` сначала объявляем локальные крейты:
```toml
[workspace.dependencies]
app = { path = "crates/app" }
core = { path = "crates/core" }
di = { path = "crates/di" }
infrastructure = { path = "crates/infrastructure" }
ui = { path = "crates/ui" }
```
А затем внешние зависимости:
```toml
gpui = "0.2.2"
gpui-component = "0.5.1"
serde = { version = "1.0", features = ["derive"] }
csv = "1.3.1"
anyhow = "1.0"
fluent-templates = "0.14" 
unic-langid = "0.9"
```

<details>
<summary>Полный код <code>Cargo.toml</code></summary>

```toml
[workspace]
members = ["crates/app", "crates/core", "crates/di", "crates/infrastructure", "crates/ui"]
default-members = ["crates/app"]
resolver = "2"

[workspace.dependencies]
jojo_stand_viewer = { path = "crates/app" }
core = { path = "crates/core" }
di = { path = "crates/di" }
infrastructure = { path = "crates/infrastructure" }
ui = { path = "crates/ui" }

gpui = "0.2.2"
gpui-component = "0.5.1"
serde = { version = "1.0", features = ["derive"] }
csv = "1.3.1"
anyhow = "1.0"
fluent-templates = "0.14" 
unic-langid = "0.9"
```

</details>

## Ассеты

В `assets` у меня следующая структура:
```
assets/
├── data/
│   └── jojo-stands.csv
├── icons/
│   ├── language.svg
│   └── theme.svg
├── images/
│   ├── Achtung Baby.png
│   ├── Aerosmith.png
│   └── ...
└── locales/
    ├── en.ftl
    └── ru.ftl
```

Здесь стоит лишь рассмотреть структуры `csv` и `ftl` файлов: <br>
`jojo-stands.csv`:
```csv
Stand,PWR,SPD,RNG,PER,PRC,DEV
Anubis,B,B,E,A,E,C
Atum,D,C,D,B,D,D
...
```
`en.ftl`:
```ftl
stand_list = Stand list

power = Power
speed = Speed
range = Range
power_persistence = Power Persistence
precision = Precision
development_potential = Development Potential
```
`ru.ftl`:
```ftl
stand_list = Список стендов

power = Сила
speed = Скорость
range = Дальность
power_persistence = Выносливость
precision = Точность
development_potential = Потенциал Развития
```

Датасет `jojo-stands.csv` я нашёл [здесь](https://www.kaggle.com/datasets/dsfelix/jojo-stands-stats?select=jojo-stands.csv).

## Локальные крейты

Теперь наконец перейдём к самой главной части, а именно к самой реализации всего того, что мы обсуждали ранее.

### Ядро

<details>
<summary>Полный код <code>core/Cargo.toml</code></summary>

```toml
[package]
name = "core"
version = "1.2.0"
edition = "2024"

[dependencies]
```

</details>

#### Типы

В директории `types` у меня реализовано лишь одно перечисление `Rank`, которое хранит информацию и возможных значения характеристик стендов:
```rust
#[derive(Debug, Clone, Copy)]
pub enum Rank {
    None,
    E,
    D,
    C,
    B,
    A,
    Infinite,
}
```

Для этого перечисления были определены функции для перевода в типы `u8` и `String`:
```rust
    pub fn to_u8(&self) -> u8 {
        match self {
            Rank::None => 0,
            Rank::E => 1,
            Rank::D => 2,
            Rank::C => 3,
            Rank::B => 4,
            Rank::A => 5,
            Rank::Infinite => 6
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Rank::None => String::from("None"),
            Rank::E => String::from("E"),
            Rank::D => String::from("D"),
            Rank::C => String::from("C"),
            Rank::B => String::from("B"),
            Rank::A => String::from("A"),
            Rank::Infinite => String::from("Infinite")
        }
    }
```

<details>
<summary>Полный код <code>core/src/types/rank.rs</code></summary>

```rust
/// Represents the statistical rank of a Stand's attribute in JoJo's Bizarre Adventure.
/// 
/// Used to classify capabilities such as Power, Speed, Range, Power Persistence, Precision, and Developmental Potential.
#[derive(Debug, Clone, Copy)]
pub enum Rank {
    None,
    E,
    D,
    C,
    B,
    A,
    Infinite,
}

impl Rank {
    /// Converts the rank into a numeric value.
    ///
    /// # Returns
    ///
    /// * A `u8` integer mapping from 0 (None) to 6 (Infinite) representing the attribute level.
    pub fn to_u8(&self) -> u8 {
        match self {
            Rank::None => 0,
            Rank::E => 1,
            Rank::D => 2,
            Rank::C => 3,
            Rank::B => 4,
            Rank::A => 5,
            Rank::Infinite => 6
        }
    }

    /// Converts the rank into a `String`.
    ///
    /// # Returns
    ///
    /// * A heap-allocated `String` containing the official text name of the rank.
    pub fn to_string(&self) -> String {
        match self {
            Rank::None => String::from("None"),
            Rank::E => String::from("E"),
            Rank::D => String::from("D"),
            Rank::C => String::from("C"),
            Rank::B => String::from("B"),
            Rank::A => String::from("A"),
            Rank::Infinite => String::from("Infinite")
        }
    }
}
```

</details>

#### Модели

В моём проекте имеется лишь одна модель, которая отвечает за конкретный стенд. В полях хранится информация о имени и характеристиках стенда:
```rust
#[derive(Debug, Clone)]
pub struct StandModel {
    name: String,
    power: Rank,
    speed: Rank,
    range: Rank,
    power_persistence: Rank,
    precision: Rank,
    development_potential: Rank
}
```

Из-за того, что приложение по своему функционалу небольшое, то как таковой сложной бизнес-логики у модели не имеется, только конструктор, геттеры и значение по умолчанию:

<details>
<summary>Полный код <code>core/src/models/stand.rs</code></summary>

```rust
use crate::types::Rank;

/// Represents a Stand with its name and statistical attributes.
#[derive(Debug, Clone)]
pub struct StandModel {
    name: String,
    power: Rank,
    speed: Rank,
    range: Rank,
    power_persistence: Rank,
    precision: Rank,
    development_potential: Rank
}

impl StandModel {
    /// Creates a new instance of a `StandModel` with the specified name and attributes.
    ///
    /// # Arguments
    ///
    /// * `name` - The unique name of the Stand.
    /// * `power` - The Power rank.
    /// * `speed` - The Speed rank.
    /// * `range` - The Range rank.
    /// * `power_persistence` - The Power Persistence rank.
    /// * `precision` - The Precision rank.
    /// * `development_potential` - The Developmental Potential rank.
    ///
    /// # Returns
    ///
    /// * An initialized `StandModel` instance containing the provided attributes.
    pub fn new(
        name: String,
        power: Rank,
        speed: Rank,
        range: Rank,
        power_persistence: Rank,
        precision: Rank,
        development_potential: Rank
    ) -> Self {
        StandModel {
            name,
            power,
            speed,
            range,
            power_persistence,
            precision,
            development_potential
        }
    }

    /// Returns a string slice referencing the Stand's name.
    ///
    /// # Returns
    ///
    /// * A string slice (`&str`) referencing the heap-allocated name of the Stand.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the Stand's Power rank.
    ///
    /// # Returns
    ///
    /// * The `Rank` enum representing the Stand's power level.
    pub fn power(&self) -> Rank {
        self.power
    }

    /// Returns the Stand's Speed rank.
    ///
    /// # Returns
    ///
    /// * The `Rank` enum representing the Stand's speed level.
    pub fn speed(&self) -> Rank {
        self.speed
    }

    /// Returns the Stand's Range rank.
    ///
    /// # Returns
    ///
    /// * The `Rank` enum representing the Stand's maximum operational distance.
    pub fn range(&self) -> Rank {
        self.range
    }

    /// Returns the Stand's Power Persistence rank.
    ///
    /// # Returns
    ///
    /// * The `Rank` enum representing the Stand's ability to maintain its state over time.
    pub fn power_persistence(&self) -> Rank {
        self.power_persistence
    }

    /// Returns the Stand's Precision rank.
    ///
    /// # Returns
    ///
    /// * The `Rank` enum representing the Stand's accuracy and combat control.
    pub fn precision(&self) -> Rank {
        self.precision
    }

    /// Returns the Stand's Developmental Potential rank.
    ///
    /// # Returns
    ///
    /// * The `Rank` enum representing the Stand's capacity to evolve or manifest new skills.
    pub fn development_potential(&self) -> Rank {
        self.development_potential
    }
}

impl Default for StandModel {
    /// Creates a default placeholder state when no Stand is currently selected.
    ///
    /// # Returns
    ///
    /// * A `StandModel` instance with the name "No selected" and all attributes set to `Rank::None`.
    fn default() -> Self {
        StandModel {
            name: String::from("No selected"),
            power: Rank::None,
            speed: Rank::None,
            range: Rank::None,
            power_persistence: Rank::None,
            precision: Rank::None,
            development_potential: Rank::None
        }
    }
}
```

</details>

#### Репозитории

У меня в качестве репозитория объявлен трейт для получения всех стендов и получения конкретного по имени.

<details>
<summary>Полный код <code>core/src/repositories/stand.rs</code></summary>

```rust
use crate::models::StandModel;

/// A thread-safe repository trait defining data access operations for Stands.
pub trait StandRepository: Send + Sync {
    /// Retrieves all available Stands from the storage source.
    ///
    /// # Returns
    ///
    /// * A `Vec<StandModel>` containing a collection of all stored Stand models.
    fn get_all(&self) -> Vec<StandModel>;

    /// Retrieves a specific Stand model by its unique name identifier.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice referencing the unique name of the Stand to search for.
    ///
    /// # Returns
    ///
    /// * An `Option<StandModel>` containing `Some(StandModel)` if a match is found, or `None` if it does not exist.
    fn get_by_name(&self, name: &str) -> Option<StandModel>;
}
```

</details>

#### Сервисы

Для того, чтобы UI мог как-либо получать информацию о стендах, нужен сервис. Сервис имеет поле с репозиторием, с помощью которого и будет доставать данные из источника:
```rust
pub struct StandService {
    stand_repository: Arc<dyn StandRepository>,
}
```

Также для сервиса должны быть методы, которые в некотором смысле являются запросами к источникам данных для их извлечения.
```rust
    pub fn get_all(&self) -> Vec<StandModel> {
        self.stand_repository.get_all()
    }

    pub fn get_by_name(&self, name: &str) -> Option<StandModel> {
        self.stand_repository.get_by_name(name)
    }
```

<details>
<summary>Полный код <code>core/src/services/stand.rs</code></summary>

```rust
use std::sync::Arc;

use crate::repositories::StandRepository;
use crate::models::StandModel;

/// A service layer structural component providing high-level business logic operations for Stands.
pub struct StandService {
    stand_repository: Arc<dyn StandRepository>,
}

impl StandService {
    /// Creates a new instance of a `StandService` injected with a thread-safe repository implementation.
    ///
    /// # Arguments
    ///
    /// * `stand_repository` - An atomically reference-counted pointer (`Arc`) wrapping a dynamic `StandRepository` trait object.
    ///
    /// # Returns
    ///
    /// * An initialized `StandService` instance.
    pub fn new(stand_repository: Arc<dyn StandRepository>) -> Self {
        Self { 
            stand_repository 
        }
    }

    /// Fetches a complete collection of all stored Stand models via the underlying repository.
    ///
    /// # Returns
    ///
    /// * A `Vec<StandModel>` containing all available Stands.
    pub fn get_all(&self) -> Vec<StandModel> {
        self.stand_repository.get_all()
    }

    /// Fetches a specific Stand model by its unique name identifier via the underlying repository.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice referencing the unique name of the Stand.
    ///
    /// # Returns
    ///
    /// * An `Option<StandModel>` containing `Some(StandModel)` if found, or `None` if no match exists.
    pub fn get_by_name(&self, name: &str) -> Option<StandModel> {
        self.stand_repository.get_by_name(name)
    }
}
```

</details>

### Инфраструктура

Мы уже закончили с ядром, поэтому можем сейчас смело переходить к слою инфраструктуры.

<details>
<summary>Полный код <code>infrastructure/Cargo.toml</code></summary>

```toml
[package]
name = "infrastructure"
version = "1.2.0"
edition = "2024"

[dependencies]
core.workspace = true

serde.workspace = true
csv.workspace = true
```

</details>

#### DTO

В проекте у меня имеется 2 DTO объекта, для типа `Rank` и для CSV строки.

Начнём с ранга, для него определяется структура, которая должна корректно считывать значения из нашего CSV файла и хранить их в нужном нам виде, то есть десериализировать:
```rust
impl<'de> Deserialize<'de> for RankDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(RankDto::from(s.as_str()))
    }
}
```

<details>
<summary>Полный код <code>infrastructure/src/dtos/rank.rs</code></summary>

```rust
use serde::{Serialize, Deserialize};

/// A Data Transfer Object (DTO) representing a Stand's statistical rank during serialization and deserialization.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum RankDto {
    None,
    E,
    D,
    C,
    B,
    A,
    Infi
}

impl From<&str> for RankDto {
    /// Maps a raw string slice representation of a rank into a matching `RankDto` variant.
    ///
    /// # Arguments
    ///
    /// * `s` - A raw string slice parsed from the data source.
    ///
    /// # Returns
    ///
    /// * The corresponding `RankDto` variant, defaulting to `RankDto::None` if the input string is unrecognized.
    fn from(s: &str) -> Self {
        match s {
            "None" => RankDto::None,
            "E" => RankDto::E,
            "D" => RankDto::D,
            "C" => RankDto::C,
            "B" => RankDto::B,
            "A" => RankDto::A,
            "Infi" => RankDto::Infi,
            _ => RankDto::None,
        }
    }
}

impl<'de> Deserialize<'de> for RankDto {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(RankDto::from(s.as_str()))
    }
}
```

</details>

Для CSV строки определяется структура, которая должна повторить поля из нашего CSV файла для корректной десериализации:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandDto {
    #[serde(rename = "Stand")]
    stand: String,
    #[serde(rename = "PWR")]
    pwr: RankDto,
    #[serde(rename = "SPD")]
    spd: RankDto,
    #[serde(rename = "RNG")]
    rng: RankDto,
    #[serde(rename = "PER")]
    per: RankDto,
    #[serde(rename = "PRC")]
    prc: RankDto,
    #[serde(rename = "DEV")]
    dev: RankDto
}
```

<details>
<summary>Полный код <code>infrastructure/src/dtos/stand.rs</code></summary>

```rust
use serde::{Serialize, Deserialize};

use super::RankDto;


/// A Data Transfer Object (DTO) used for flat-file deserialization and serialization of Stand records.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandDto {
    #[serde(rename = "Stand")]
    stand: String,
    #[serde(rename = "PWR")]
    pwr: RankDto,
    #[serde(rename = "SPD")]
    spd: RankDto,
    #[serde(rename = "RNG")]
    rng: RankDto,
    #[serde(rename = "PER")]
    per: RankDto,
    #[serde(rename = "PRC")]
    prc: RankDto,
    #[serde(rename = "DEV")]
    dev: RankDto
}

impl StandDto {
    /// Creates a new instance of a `StandDto` with explicit raw transfer components.
    ///
    /// # Arguments
    ///
    /// * `stand` - The raw name string of the Stand.
    /// * `pwr` - The mapped raw power rank token.
    /// * `spd` - The mapped raw speed rank token.
    /// * `rng` - The mapped raw range rank token.
    /// * `per` - The mapped raw persistence rank token.
    /// * `prc` - The mapped raw precision rank token.
    /// * `dev` - The mapped raw development potential rank token.
    ///
    /// # Returns
    ///
    /// * An initialized `StandDto` transfer state payload container.
    pub fn new(
        stand: String,
        pwr: RankDto,
        spd: RankDto,
        rng: RankDto,
        per: RankDto,
        prc: RankDto,
        dev: RankDto
    ) -> Self {
        StandDto {
            stand,
            pwr,
            spd,
            rng,
            per,
            prc,
            dev
        }
    }

    /// Accesses the raw name of the Stand.
    ///
    /// # Returns
    ///
    /// * A string slice referencing the internal name identifier.
    pub fn stand(&self) -> &str {
        &self.stand
    }

    /// Accesses the raw short power rank data token.
    ///
    /// # Returns
    ///
    /// * The `RankDto` enumeration token value for power.
    pub fn power(&self) -> RankDto {
        self.pwr
    }

    /// Accesses the raw short speed rank data token.
    ///
    /// # Returns
    ///
    /// * The `RankDto` enumeration token value for speed.
    pub fn speed(&self) -> RankDto {
        self.spd
    }

    /// Accesses the raw short range rank data token.
    ///
    /// # Returns
    ///
    /// * The `RankDto` enumeration token value for range.
    pub fn range(&self) -> RankDto {
        self.rng
    }

    /// Accesses the raw short power persistence rank data token.
    ///
    /// # Returns
    ///
    /// * The `RankDto` enumeration token value for power persistence.
    pub fn power_persistence(&self) -> RankDto {
        self.per
    }

    /// Accesses the raw short precision rank data token.
    ///
    /// # Returns
    ///
    /// * The `RankDto` enumeration token value for precision.
    pub fn precision(&self) -> RankDto {
        self.prc
    }

    /// Accesses the raw short developmental potential rank data token.
    ///
    /// # Returns
    ///
    /// * The `RankDto` enumeration token value for developmental potential.
    pub fn development_potential(&self) -> RankDto {
        self.dev
    }
}
```

</details>

#### Мапперы

Для начала определим трейт `Mapper`, который реализует методы перевода в модель и в DTO.

<details>
<summary>Полный код <code>infrastructure/src/mappers/mapper.rs</code></summary>

```rust
/// A generic data mapping abstraction layer used to convert data structures between different architectural layers.
///
/// Typically implemented within the infrastructure layer to decouple core domain models from external Data Transfer Objects (DTOs).
///
/// # Type Parameters
///
/// * `Model` - The core domain model representing business logic rules.
/// * `Dto` - The data transfer representation..
pub trait Mapper<Model, Dto> {
    /// Maps a core domain model reference into its data transfer object counterpart.
    ///
    /// # Arguments
    ///
    /// * `model` - A shared reference (`&Model`) to the core entity being converted.
    ///
    /// # Returns
    ///
    /// * An instance of the corresponding `Dto`.
    fn to_dto(model: &Model) -> Dto;

    /// Maps a data transfer object reference back into its core domain model representation.
    ///
    /// # Arguments
    ///
    /// * `dto` - A shared reference (`&Dto`) to the data transfer object layout being converted.
    ///
    /// # Returns
    ///
    /// * An instance of the corresponding core `Model`.
    fn to_model(dto: &Dto) -> Model;
}
```

</details>

А далее определяем мапперы для ранга и стенда.

<details>
<summary>Полный код <code>infrastructure/src/mappers/rank.rs</code></summary>

```rust
use core::types::Rank;
use crate::dtos::RankDto;
use super::mapper::Mapper;

/// A stateless mapping utility responsible for converting between domain `Rank` enums and infrastructure `RankDto` representations.
pub struct RankMapper;

impl Mapper<Rank, RankDto> for RankMapper {
    /// Maps a domain `Rank` reference into its infrastructure-specific `RankDto` counterpart.
    ///
    /// # Arguments
    ///
    /// * `model` - A shared reference to the domain `Rank` enum variant.
    ///
    /// # Returns
    ///
    /// * The corresponding `RankDto` enum variant used for transfer layouts.
    fn to_dto(model: &Rank) -> RankDto {
        match model {
            Rank::None => RankDto::None,
            Rank::E => RankDto::E,
            Rank::D => RankDto::D,
            Rank::C => RankDto::C,
            Rank::B => RankDto::B,
            Rank::A => RankDto::A,
            Rank::Infinite => RankDto::Infi
        }
    }

    /// Maps an infrastructure `RankDto` reference back into its domain `Rank` representation.
    ///
    /// # Arguments
    ///
    /// * `dto` - A shared reference to the `RankDto` data serialization variant.
    ///
    /// # Returns
    ///
    /// * The corresponding core domain `Rank` enum variant.
    fn to_model(dto: &RankDto) -> Rank {
        match dto {
            RankDto::None => Rank::None,
            RankDto::E => Rank::E,
            RankDto::D => Rank::D,
            RankDto::C => Rank::C,
            RankDto::B => Rank::B,
            RankDto::A => Rank::A,
            RankDto::Infi => Rank::Infinite
        }
    }
}
```

</details>

<details>
<summary>Полный код <code>infrastructure/src/mappers/stand.rs</code></summary>

```rust
use core::models::StandModel;
use crate::dtos::StandDto;
use super::mapper::Mapper;
use crate::mappers::RankMapper;

/// A stateless mapping utility responsible for converting between complex domain `StandModel` structures and flattened infrastructure `StandDto` data layers.
pub struct StandMapper;

impl Mapper<StandModel, StandDto> for StandMapper {
    /// Maps a domain `StandModel` reference into its infrastructure-specific serialized `StandDto` layout.
    ///
    /// # Arguments
    ///
    /// * `model` - A shared reference to the domain `StandModel`.
    ///
    /// # Returns
    ///
    /// * A fully initialized `StandDto` instance containing mapped attribute transfer tokens.
    fn to_dto(model: &StandModel) -> StandDto {
        StandDto::new(
            String::from(model.name()),
            RankMapper::to_dto(&model.power()),
            RankMapper::to_dto(&model.speed()),
            RankMapper::to_dto(&model.range()),
            RankMapper::to_dto(&model.power_persistence()),
            RankMapper::to_dto(&model.precision()),
            RankMapper::to_dto(&model.development_potential())
        )
    }

    /// Maps a raw infrastructure `StandDto` data transfer record back into a domain `StandModel`.
    ///
    /// # Arguments
    ///
    /// * `dto` - A shared reference to the deserialized external `StandDto` data block.
    ///
    /// # Returns
    ///
    /// * A core domain `StandModel` instance.
    fn to_model(dto: &StandDto) -> StandModel {
        StandModel::new(
            String::from(dto.stand()),
            RankMapper::to_model(&dto.power()),
            RankMapper::to_model(&dto.speed()),
            RankMapper::to_model(&dto.range()),
            RankMapper::to_model(&dto.power_persistence()),
            RankMapper::to_model(&dto.precision()),
            RankMapper::to_model(&dto.development_potential())
        )
    }
}
```

</details>

#### Репозитории

Здесь наконец будет реализована имплементация репозитория для нашего CSV файла. Для его создания я воспользовался крейтом `csv`, чтобы считать данные и манипулировать ими.

<details>
<summary>Полный код <code>infrastructure/src/repositories/csv_stand.rs</code></summary>

```rust
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::Reader;

use core::models::StandModel;
use core::repositories::StandRepository;
use crate::dtos::StandDto;
use crate::mappers::{Mapper,StandMapper};
use crate::file::PathManager;

pub struct CsvStandRepository {
    items: Vec<StandModel>,
}

/// An implementation of `StandRepository` that loads and caches data from a flat CSV file.
impl CsvStandRepository {
    /// Initializes the repository by parsing records from a CSV file.
    ///
    /// # Arguments
    ///
    /// * `path_manager` - A shared reference to the utility managing environment file paths.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the initialized `CsvStandRepository` state on success, or a dynamic wrapper of any IO/Parsing error.
    pub fn new(path_manager: &PathManager) -> Result<Self, Box<dyn Error>> {
        let mut items = Vec::new();

        let path = path_manager.csv_path();
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut rdr = Reader::from_reader(reader);

        for result in rdr.deserialize() {
            let record: StandDto = result?;
            items.push(StandMapper::to_model(&record));
        }

        Ok(Self { items })
    }
}

impl StandRepository for CsvStandRepository {
    /// Returns a cloned snapshot collection of all cached domain models.
    ///
    /// # Returns
    ///
    /// * A `Vec<StandModel>` populated with copies of the Stand entities.
    fn get_all(&self) -> Vec<StandModel> {
        self.items.clone()
    }

    /// Searches for a specific Stand by its unique string name in the local cached vector.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice referencing the Stand's unique lookup key.
    ///
    /// # Returns
    ///
    /// * An `Option<StandModel>` containing a cloned model instance if matched, or `None`.
    fn get_by_name(&self, name: &str) -> Option<StandModel> {
        self.items.iter().find(|s| s.name() == name).cloned()
    }
}
```

</details>

#### Компоненты для работы с файлами

В своём проекте я определил структуру для управления путями файлов `PathManager`, которая предназначена для получения путей к ассетам. Также будь у меня в проекте много иконок, то можно было бы также определить `IconManager`, для более удобного менеджмента иконками.

<details>
<summary>Полный код <code>infrastructure/src/file/path_manager.rs</code></summary>

```rust
use std::path::{Path, PathBuf};

/// A utility structure responsible for managing component file paths across the application asset space.
///
/// It constructs and verifies path locations for static data, icons, and dynamic stand images.
pub struct PathManager {
    assets_dir: PathBuf,
    csv_path: PathBuf
}

impl PathManager {
    /// Constructs a new `PathManager` by combining a base directory with specific resource names.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - An object that implements `AsRef<Path>` serving as the root directory of the project.
    /// * `assets_dir_name` - The subfolder name string holding static resources.
    /// * `csv_name` - The base name of the database file without the extension.
    ///
    /// # Returns
    ///
    /// * An initialized `PathManager` with calculated asset and database directory trees.
    pub fn new(base_dir: impl AsRef<Path>, assets_dir_name: &str, csv_name: &str) -> Self {
        let base_dir = base_dir.as_ref().to_path_buf();
        let assets_dir = base_dir.join(assets_dir_name);

        let csv_filename = format!("{}.csv", csv_name);
        let csv_path = assets_dir.join("data").join(csv_filename);

        PathManager {
            assets_dir,
            csv_path
        }
    }

    /// Accesses the root path of the asset directory tree.
    ///
    /// # Returns
    ///
    /// * A shared reference to a `Path` pointing to the application's root asset directory.
    pub fn assets_dir(&self) -> &Path {
        &self.assets_dir
    }

    /// Accesses the designated CSV flat-file path.
    ///
    /// # Returns
    ///
    /// * A shared reference to a `Path` pointing directly to the target CSV database file.
    pub fn csv_path(&self) -> &Path {
        &self.csv_path
    }

    /// Resolves the absolute runtime path for a requested Stand profile image.
    ///
    /// If the specific image file does not exist on disk, it falls back to a default placeholder image.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice referencing the relative path/filename of the stand image.
    ///
    /// # Returns
    ///
    /// * A `PathBuf` container holding the verified image file path or the "unknown.png" fallback location.
    pub fn image_path(&self, path: &str) -> PathBuf {
        let image_path =  self.assets_dir.join("images").join(path);
        if image_path.exists() {
            image_path
        } else {
            self.assets_dir.join("images").join("unknown.png")
        }
    }

    /// Resolves the file path for graphical asset icons.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice referencing the relative path or file descriptor of the icon.
    ///
    /// # Returns
    ///
    /// * A `PathBuf` holding the absolute directory layout path for the icon file.
    pub fn icon_path(&self, path: &str) -> PathBuf {
        self.assets_dir.join("icons").join(path)
    }
}
```

</details>

### DI

Так как мы определили всё необходимое для логики и работы с источниками данных, то теперь можем переходить к DI.

<details>
<summary>Полный код <code>di/Cargo.toml</code></summary>

```toml
[package]
name = "di"
version = "1.2.0"
edition = "2024"

[dependencies]
core.workspace = true
infrastructure.workspace = true

gpui.workspace = true
```

</details>

Моя структура `DependencyInjector` инициализирует `PathManager` и `StandService` и даёт доступ к их реализациям, так как эти объекты должны быть доступны из каждого UI-элемента.
```rust
    pub fn init(base_dir: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let path_manager = Arc::new(PathManager::new(base_dir, "assets", "jojo-stands"));

        let csv_stand_repository = CsvStandRepository::new(&path_manager)?;
        let stand_repository: Arc<dyn StandRepository> = Arc::new(csv_stand_repository);
        let stand_service = Arc::new(StandService::new(stand_repository));

        Ok(Self {
            path_manager,
            stand_service
        })
    }
```

Для `DependencyInjector` также определена имплементация `gpui::Global`, это необходимо, чтобы контекст в GPUI мог его хранить в качестве глобального объекта, доступного из любой части, где определён контекст.
```rust
impl Global for DependencyInjector {}
```

В связи с тем, что мне нет необходимости инициализировать здесь много элементов, то в DI слое у меня всего одна структура, однако если у Вас крупный проект, где необходима инициализация огромного числа структур, то будет удобнее, если Вы создадите подмодули, где отдельно инициализируются компоненты из разных слоёв, и потом объедините все эти подмодули в `DependencyInjector`.

<details>
<summary>Полный код <code>di/src/dependency_injector.rs</code></summary>

```rust
use std::sync::Arc;
use std::path::Path;
use gpui::Global;

use core::repositories::StandRepository;
use core::services::StandService;
use infrastructure::file::PathManager;
use infrastructure::repositories::CsvStandRepository;

/// A centralized dependency injection container that manages the application's global state lifetimes.
pub struct DependencyInjector {
    path_manager: Arc<PathManager>,
    stand_service: Arc<StandService>,
}

impl Global for DependencyInjector {}

impl DependencyInjector {
    /// Initializes the entire application dependency graph by assembling infrastructure and core service layers.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - An object implementing `AsRef<Path>` that points to the application's root execution directory.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the fully wired `DependencyInjector` container on success, or an IO/parsing startup error wrapped in a `Box`.
    pub fn init(base_dir: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let path_manager = Arc::new(PathManager::new(base_dir, "assets", "jojo-stands"));

        let csv_stand_repository = CsvStandRepository::new(&path_manager)?;
        let stand_repository: Arc<dyn StandRepository> = Arc::new(csv_stand_repository);
        let stand_service = Arc::new(StandService::new(stand_repository));

        Ok(Self {
            path_manager,
            stand_service
        })
    }

    /// Provides a thread-safe, reference-counted clone of the `PathManager` service.
    ///
    /// # Returns
    ///
    /// * An `Arc<PathManager>` pointer managing asset paths.
    pub fn path_manager(&self) -> Arc<PathManager> {
        self.path_manager.clone()
    }

    /// Provides a thread-safe, reference-counted clone of the `StandService`.
    ///
    /// # Returns
    ///
    /// * An `Arc<StandService>` pointer used for performing application operations.
    pub fn stand_service(&self) -> Arc<StandService> {
        self.stand_service.clone()
    }
}
```

</details>

### UI

Мы уже на финишной прямой! Наконец поговорим о том, как у меня устроена часть пользовательского интерфейса.

<details>
<summary>Полный код <code>ui/Cargo.toml</code></summary>

```toml
[package]
name = "ui"
version = "1.2.0"
edition = "2024"

[dependencies]
core.workspace = true
infrastructure.workspace = true
di.workspace = true

gpui.workspace = true
gpui-component.workspace = true
fluent-templates.workspace = true
unic-langid.workspace = true
```

</details>

#### Тема

В своём проекте я определил лишь 2 темы: светлая и тёмная. Я создал перечисление для более удобной манипуляции этой информацией.
```rust
#[derive(Clone)]
enum ThemeMode {
    Light,
    Dark
}
```

Для структуры `Theme` я определил поля для определения цветов разных частей UI и поле для хранении информации о выбранной теме.
```rust
#[derive(Clone)]
pub struct Theme {
    mode: ThemeMode,

    pub background_color: u32,
    pub text_color: u32,

    pub button_color: u32,
    pub button_hover_color: u32,

    pub grid_color: u32,
    pub polygon_color: u32,
    pub polygon_opacity: u8,
    pub radar_text_color: Hsla,
}
```

В качестве методов были определены `light` и `dark`, которые являются фактически конструкторами, создающими структуру с указанными в методе значениями.

Также небольшой метод для смены темы. Здесь используется переменная контекста `cx`. С помощью `cx.set_global(new_theme)` в контексте структура типа `Theme` меняется на новую со значениями `new_theme`. А `cx.refresh_windows()` принудительно перерисовывает все окна, чтобы применить новое значение, которое мы ранее определили.
```rust
    pub fn toggle_theme(&self, cx: &mut App) {
        let current = cx.global::<Self>().clone();

        let new_theme = match current.mode {
            ThemeMode::Light => Self::dark(),
            ThemeMode::Dark => Self::light(),
        };

        cx.set_global(new_theme);
        cx.refresh_windows();
    }
```

<details>
<summary>Полный код <code>ui/src/theme.rs</code></summary>

```rust
use gpui::{App, Global, Hsla, hsla};

/// Represents the active visual mode of the application's user interface.
#[derive(Clone)]
enum ThemeMode {
    Light,
    Dark
}

/// A comprehensive styling configuration containing color palettes and opacity settings for the UI.
#[derive(Clone)]
pub struct Theme {
    mode: ThemeMode,

    pub background_color: u32,
    pub text_color: u32,

    pub button_color: u32,
    pub button_hover_color: u32,

    pub grid_color: u32,
    pub polygon_color: u32,
    pub polygon_opacity: u8,
    pub radar_text_color: Hsla,
}

impl Global for Theme {}

impl Theme {
    /// Constructs a predefined dark theme configuration matching low-light environment palettes.
    ///
    /// # Returns
    ///
    /// * A `Theme` instance initialized with dark background colors and high-contrast text markers.
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,

            background_color: 0x1e1e1e,
            text_color: 0xffffff,

            button_color: 0x2d2d2d,
            button_hover_color: 0x37373d,

            grid_color: 0x9d9d9d,
            polygon_color: 0xff830f,
            polygon_opacity: 120,
            radar_text_color: hsla(0.0, 1.0, 0.99, 1.0),
        }
    }

    /// Constructs a predefined light theme configuration matching high-luminance environment palettes.
    ///
    /// # Returns
    ///
    /// * A `Theme` instance initialized with light background colors and readable dark accents.
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,

            background_color: 0xffffff,
            text_color: 0x1e1e1e,

            button_color: 0xdadada,
            button_hover_color: 0xb7b7b7,

            grid_color: 0xafa9a9,
            polygon_color: 0xff830f,
            polygon_opacity: 120,
            radar_text_color: hsla(0.0, 0.0, 0.0, 1.0),
        }
    }

    /// Toggles the global application theme between Light and Dark modes.
    ///
    /// # Arguments
    ///
    /// * `cx` - A mutable reference to the GPUI `App` context.
    pub fn toggle_theme(&self, cx: &mut App) {
        let current = cx.global::<Self>().clone();

        let new_theme = match current.mode {
            ThemeMode::Light => Self::dark(),
            ThemeMode::Dark => Self::light(),
        };

        cx.set_global(new_theme);
        cx.refresh_windows();
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
```

</details>

#### Локализация

Для работы с файлами `ftl` я воспользовался крейтами `fluent_templates` и `unic_langid`.

Для начала я вызвал макрос `static_loader`, он на загружает все fluent ресурсы на этапе компиляции, а также задаёт язык по умолчанию.
```rust
static_loader! {
    pub static LOCALES = {
        locales: "../../assets/locales",
        fallback_language: "en",
    };
}
```

Естественно я реализовал структуру, которая хранит в себе текущий язык и определяет трейт `gpui::Global`.
```rust
#[derive(Clone)]
pub struct Locale {
    pub language: LanguageIdentifier,
}

impl Global for Locale {}
```

Также я реализовал метод для перевода строки по переданному ключу из `ftl` файлу и метод для смены языка, который схож в реализации с сменой темы.
```rust
pub fn translate(&self, key: &str) -> String {
        LOCALES.try_lookup(&self.language, key)
            .unwrap_or_else(|| {
                println!("[Localization Warning] Key '{}' not found!", key);
                key.to_string() 
            })
    }

    pub fn toggle_language(&self, cx: &mut gpui::App) {
        let locale = cx.global::<Self>().clone();
        
        let new_language = if locale.language == langid!("ru") {
            langid!("en")
        } else {
            langid!("ru")
        };

        cx.set_global(Locale { language: new_language });
        cx.refresh_windows();
    }
```

Для удобства, дабы не прописывать в компонентах огромную строку кода, чтобы просто указать текст в UI, я определил функцию которая позволит получать строку с более удобным синтаксисом `tr(cx, "key")`.
```rust
pub fn tr(cx: &gpui::App, key: &str) -> gpui::SharedString {
    cx.global::<Locale>().translate(key).into()
}
```

<details>
<summary>Полный код <code>ui/src/locale.rs</code></summary>

```rust
use gpui::Global;
use fluent_templates::{Loader, static_loader};
use unic_langid::{langid, LanguageIdentifier};

static_loader! {
    pub static LOCALES = {
        locales: "../../assets/locales",
        fallback_language: "en",
    };
}

/// A global localization manager responsible for translating UI strings and managing the application language state.
#[derive(Clone)]
pub struct Locale {
    pub language: LanguageIdentifier,
}

impl Global for Locale {}

impl Locale {
    /// Constructs a new `Locale` manager initialized with English ("en") as the default language.
    ///
    /// # Returns
    ///
    /// * An initialized `Locale` state instance.
    pub fn new() -> Self {
        Self {
            language: langid!("en"),
        }
    }

    /// Looks up a localization translation resource string matching the provided key identifier.
    ///
    /// If the key is missing in the primary and fallback languages, it prints a warning to the console and safely returns the raw key string to avoid UI crashes.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice referencing the unique translation key.
    ///
    /// # Returns
    ///
    /// * A resolved heap-allocated `String` translation text.
    pub fn translate(&self, key: &str) -> String {
        LOCALES.try_lookup(&self.language, key)
            .unwrap_or_else(|| {
                println!("[Localization Warning] Key '{}' not found!", key);
                key.to_string() 
            })
    }

    /// Toggles the active application language between Russian and English layouts.
    ///
    /// # Arguments
    ///
    /// * `cx` - A mutable reference to the global GPUI `App` application instance context.
    pub fn toggle_language(&self, cx: &mut gpui::App) {
        let locale = cx.global::<Self>().clone();
        
        let new_language = if locale.language == langid!("ru") {
            langid!("en")
        } else {
            langid!("ru")
        };

        cx.set_global(Locale { language: new_language });
        cx.refresh_windows();
    }
}

/// A short global utility helper function used to fetch localized strings swiftly.
///
/// # Arguments
///
/// * `cx` - A shared reference to the GPUI application lifecycle state context.
/// * `key` - The unique translation dictionary lookup identifier key token.
///
/// # Returns
///
/// * A thread-safe, immutable `SharedString` compatible with GPUI primitive rendering components.
pub fn tr(cx: &gpui::App, key: &str) -> gpui::SharedString {
    cx.global::<Locale>().translate(key).into()
}
```

</details>

#### Общие компоненты

Начиная с этого момента я больше буду рассказывать про возможности самого GPUI и меньше про сам мой проект. Если Вы хотите увидеть оставшуюся часть моего проекта более подробно, то в конце статьи я оставлю ссылку на GitHub с репозиторием. 

В моём проекте определён лишь один общий компонент: кнопка с возможностью стилизации. Я решил, что моя кнопка внутри себя может хранить либо текст, либо SVG иконку в качестве дочернего элемента.
```rust
pub enum ButtonContentType {
    Text(SharedString),
    Icon(String, f32),
}

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    content_type: ButtonContentType,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    modifier: Option<Box<dyn FnOnce(Stateful<Div>) -> Stateful<Div> + 'static>>
}

impl Button {
    pub fn on_click(mut self, on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(on_click));
        self
    }

    pub fn style_modifier(mut self, modifier: impl FnOnce(Stateful<Div>) -> Stateful<Div> + 'static) -> Self {
        self.modifier = Some(Box::new(modifier));
        self
    }
}
```

Так как кнопка является переиспользуемым объектом, то должен определять трейт `RenderOnce`. То, как будет выглядеть наш UI-элемент мы определяем как раз в методе `render`. 
```rust
impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
```

Переменная `window` нужна для работы с окном приложения, `cx` мы уже применяли ранее, когда определяли тему и локализацию, это наш контекст приложения. Конструктция `cx.global::<G>()` позволяет получить из контекста глобальный объект типа `G`.

Стлизация интерфеса, по словам команды Zed, вдохновлена API Tailwind CSS, поэтому если Вы знакомы с ним или с обычными CSS и HTML, то синтаксис должен быть Вам знаком. 

Большинство элементов будут являться конструкциями `Div`, который, как и в HTML, является контейнером для других элементов. Также важный момент, что для всех элементов, которые поддержмвают методы для взаимодействия с пользователем, должны обладать уникальным идентификатором, который задаётся через метод `id`. Подробно про методы для стилизации элементов я описывать не буду, многие из них понятны по названию, да и в официальной документации они все хорошо и пнтяно описаны.
```rust
        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .rounded_md()
            .p_2()
```

Метод `when_some` выполняет функцию, которую в него передали, если переменная определена.
```rust
            .when_some(self.modifier, |this, modifier| {
                modifier(this)
            })
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |evt, window, cx| (on_click)(evt, window, cx))
            })
```
Здесь метод `modifier` я определил для того, чтобы можно было задавать стили для кнопки, когда она будет вызываться в других компонентах. Это будет выглядеть примерно так:
```rust
button(
    "some-id", 
    ButtonContentType::Text("Text")
)
.style_modifier(move |style| {
    style
        .w_full()
        .h(px(40.0))
        .bg(rgb(0x000000))
        // И остальные стили, которые зададите
})
```

Метод `child` нужен, дабы задать дочерние элементы, имненно в нём определяется основной контент UI-элемента. Если в качестве аргумента определить строку, то в приложении будет отрисован переданный текст с определёнными стилями текста.
```rust
div()
    .child("Text")
    .child(
        div()
    )
```

Также я думаю, будет удобно, если для ___общих компонент___ вместо конструктора `new` определить глобальную фабричную функцию. Это позволит не только определять UI в более привычном и общем виде, но и наглядно отличать ___общие компоненты___ от ___внутренних___.
```rust
pub fn button(id: impl Into<ElementId>, content_type: ButtonContentType) -> Button {
    Button { 
        id: id.into(),
        content_type,
        on_click: None,
        modifier: None
    }
}
```

<details>
<summary>Полный код <code>ui/src/shared/button.rs</code></summary>

```rust
use gpui::{
    SharedString, IntoElement, ElementId, ClickEvent,
    Window, App, Stateful, Div, RenderOnce, InteractiveElement,
    Styled, StatefulInteractiveElement, ParentElement,
    div, svg, px, rgb,
    prelude::FluentBuilder
};

use di::DependencyInjector;
use crate::Theme;

/// Defines the visual and structural payload inside the custom UI button.
pub enum ButtonContentType {
    Text(SharedString),
    Icon(String, f32),
}

/// A highly reusable, composable, and customizable stateful component built on top of GPUI primitives.
///
/// It supports fluid functional styling adjustments via custom modifier closures and lifecycle interaction event hooks.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    content_type: ButtonContentType,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    modifier: Option<Box<dyn FnOnce(Stateful<Div>) -> Stateful<Div> + 'static>>
}

impl Button {
    /// Registers an asynchronous interaction callback executed whenever the user dispatches a pointer click event.
    ///
    /// # Arguments
    ///
    /// * `on_click` - A closure triggered on execution, mutating the parent frame architecture states.
    ///
    /// # Returns
    ///
    /// * The updated `Button` builder instance.
    pub fn on_click(mut self, on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(on_click));
        self
    }

    /// Appends external decorative style overrides onto the baseline layout wrapper tree state.
    ///
    /// # Arguments
    ///
    /// * `modifier` - A closure taking ownership of the existing stateful element, returning the stylized version.
    ///
    /// # Returns
    ///
    /// * The updated `Button` builder instance.
    pub fn style_modifier(mut self, modifier: impl FnOnce(Stateful<Div>) -> Stateful<Div> + 'static) -> Self {
        self.modifier = Some(Box::new(modifier));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let path_manager = cx.global::<DependencyInjector>().path_manager();
        let theme = cx.global::<Theme>();

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .rounded_md()
            .p_2()
            .when_some(self.modifier, |this, modifier| {
                modifier(this)
            })
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |evt, window, cx| (on_click)(evt, window, cx))
            })
            .child(match self.content_type {
                ButtonContentType::Text(text) => div().child(text).into_any_element(),
                ButtonContentType::Icon(icon, size) => {
                    let path = path_manager
                        .icon_path(&icon)
                        .to_string_lossy()
                        .into_owned();

                    svg()
                        .size(px(size))
                        .path(path)
                        .text_color(rgb(theme.text_color))
                        .into_any_element()
                },
            })
    }
}

/// A global functional initialization helper used to generate standalone standard interactive `Button` primitives.
///
/// # Arguments
///
/// * `id` - A unique identifier token compatible with GPUI element state matching rules.
/// * `content_type` - The state token describing what information payload the button displays internally.
///
/// # Returns
///
/// * A baseline configured `Button` component struct instance.
pub fn button(id: impl Into<ElementId>, content_type: ButtonContentType) -> Button {
    Button { 
        id: id.into(),
        content_type,
        on_click: None,
        modifier: None
    }
}
```

</details>

#### Фичи

Как уже упоминалось ранее, к ___фичам___ будут относиться окна, экраны и те UI-элементы, которые реализуют сложную логику, которая меняет внешний вид при взаимодействия пользователя с ним. Насчёт последнего, наверное, стоит поговрить чуть больше. К примеру, Вам могло показаться, что к последней категории ___фич___, может относиться кнопка для смены темы. Так как поля темы являются глобальными для всего экрана, то данное действие должно менять не только саму кнопку, но и всё остальное, в связи с чем данная кнопка будет попросту ___внутренним компонентом___ экрана, который уже является ___фичей___.

В моём проекте имеется лишь одна ___фича___ $-$ это главный, и единственный, экран. Если бы я хотел ещё дополнить проект, допустим, добавлением возможности поиска, то панель с поисковым полем и списком подходящих кнопок было бы ___фичей___, так как в зависимости от действия пользователя в текстовом поле будут изменяться содержащийся текст, позиция курсора и список стендов. Изменения этой ___фичи___ никак не относятся ко всему экрану, они локализованы внутри данной панели.

Теперь же перейдём к реализации ___фичи___. 

##### Состояние

___Состояние___ должно хранить в себе поля данных, которые актуальны для ___фичи___, и выполнять логику изменения этих полей. То бишь оно должно быть максимально отстранено от UI, выполнять лишь хранение и логику. ___Вид___ будет изменяться при изменении ___состояния___

<details>
<summary>Полный код <code>ui/src/features/main_screen/state.rs</code></summary>

```rust
use std::sync::Arc;

use core::services::StandService;
use core::models::StandModel;

/// Represents the presentation and interactive state container for the main screen.
pub struct MainScreenState {
    stand_service: Arc<StandService>,
    stands: Vec<StandModel>,
    selected_stand_name: Option<String>,
}

impl MainScreenState {
    /// Constructs a new `MainScreenState`, instantly pre-fetching the available record sets.
    ///
    /// # Arguments
    ///
    /// * `stand_service` - A thread-safe shared reference pointer to the core `StandService` instance.
    ///
    /// # Returns
    ///
    /// * An initialized state machine payload with no initial selection targeted.
    pub fn new(stand_service: Arc<StandService>) -> Self {
        let stands = stand_service.get_all();

        Self {
            stand_service,
            stands,
            selected_stand_name: None
        }
    }

    /// Provides a slice reference of all pre-cached `StandModel` entities.
    ///
    /// # Returns
    ///
    /// * A slice sequence containing compiled model payloads.
    pub fn stands(&self) -> &[StandModel] {
        &self.stands
    }

    /// Exposes a shared reference to the unique identifier string of the currently active selection.
    ///
    /// # Returns
    ///
    /// * An `Option` reference wrapping the selected target item name key.
    pub fn selected_stand_name(&self) -> &Option<String> {
        &self.selected_stand_name
    }

    /// Resolves and fetches the full model payload corresponding to the active user selection state identifier.
    ///
    /// It queries the business service layer dynamically based on the stored text key index.
    ///
    /// # Returns
    ///
    /// * `Some(StandModel)` if an identity key is selected and verified by the repository backend, or `None`.
    pub fn selected_stand(&self) -> Option<StandModel> {
        let selected_name = self.selected_stand_name.clone();
        match selected_name {
            None => None,
            Some(name) => match self.stand_service.get_by_name(&name.as_str()) {
                None => None,
                Some(stand) => Some(stand)
            }
        }
    }

    /// Mutates the selection index parameters to target a new focused record entry point.
    ///
    /// # Arguments
    ///
    /// * `name` - The concrete string name identity token representing the newly focused model item.
    pub fn select_stand(&mut self, name: String) {
        self.selected_stand_name = Some(name);
    }
}
```

</details>

##### Вид

Вид обязательно обладает полями типов `gpui::Entity<SomeState>` и `gpui::Subscription`. `Entity` является некоторой прослойкой, которая управляет ___видом___ в зависимости от того, как меняется ___состояние___. А `Subscription` является токеном подписки, который сохраняет связь, пока существует.
```rust
pub struct MainScreen {
    entity: Entity<MainScreenState>,
    _subscription: Subscription
}

impl MainScreen {
    pub fn new(entity: Entity<MainScreenState>, cx: &mut Context<Self>) -> Self {
        let _subscription = cx.observe(&entity, |_, _, cx| cx.notify());
        MainScreen {
            entity,
            _subscription
        }
    }
}
```
`cx.observe()` как раз и создаёт объект `Subscription`, а `cx.notify()` уведомляет ___вид___, что `Entity` изменён.

Также стоит упомянуть, что не стоит передавать `Entity` или ___состояние___ во ___внутренние компоненты___, доступ к ним должен иметь лишь сам ___вид___, во ___внутренние компоненты___ мы можем передавать сами поля ___состояния___ и замыкания с вызовом методов ___состояния___.
```rust
.child(
    Sidebar::new(stands.to_vec(), move |stand_name, _window, cx| {
        state_clone.update(cx, |state, _cx| {
            state.select_stand(stand_name); 
        });
    })
)
```

<details>
<summary>Полный код <code>ui/src/features/main_screen/view.rs</code></summary>

```rust
use gpui::{
    Entity, Subscription, Render, Window, Context,
    IntoElement, Styled, ParentElement,
    div, rgb
};

use super::MainScreenState;
use super::components::Sidebar;
use super::components::StandInfo;
use crate::Theme;

/// The UI view container component representing the main screen.
pub struct MainScreen {
    entity: Entity<MainScreenState>,
    _subscription: Subscription
}

impl MainScreen {
    /// Constructs a new `MainScreen` instance and binds a change observation listener.
    ///
    /// # Arguments
    ///
    /// * `entity` - An encapsulated GPUI `Entity` holding the reactive model state data logic.
    /// * `cx` - A mutable reference to the view's current operational execution context.
    ///
    /// # Returns
    ///
    /// * An initialized view layout node subscribing to state events.
    pub fn new(entity: Entity<MainScreenState>, cx: &mut Context<Self>) -> Self {
        let _subscription = cx.observe(&entity, |_, _, cx| cx.notify());
        MainScreen {
            entity,
            _subscription
        }
    }
}

impl Render for MainScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.entity.read(cx);
        let stands = state.stands();
        let state_clone = self.entity.clone();
        let selected_stand = state.selected_stand();

        let theme = cx.global::<Theme>();

        div()
            .size_full()
            .flex()
            .bg(rgb(theme.background_color))
            .text_color(rgb(theme.text_color))
            .child(
                Sidebar::new(stands.to_vec(), move |stand_name, _window, cx| {
                    state_clone.update(cx, |state, _cx| {
                        state.select_stand(stand_name); 
                    });
                })
            )
            .child(
                StandInfo::new(selected_stand)
            )
    }
}
```

</details>


##### Внутренние компоненты 

В этом разделе я буду по большей части рассказывать не про сам проект, а уже про остальные возможности GPUI, которые мною применялись. Если хотите рассомтореть всё это дело более подробно, то лучше посмотрите репозиторий проекта.

Также в какой-то момент, когда пытался реализовать скролл в GPUI, я наткнулся на довольно полезный крейт `gpui-component`, в котором реализовано огромное число базовых и не очень компонентов, включая сам скролл. Вот [ссылка](https://longbridge.github.io/gpui-component/) на него. Для реализации скролла я применил к структуре `Stateful<Div>` метод `overflow_y_scroll()`, который добавляет возможность скроллинга по вертикали.
```rust
v_flex()
    .id("stand_list")
    .overflow_y_scroll()
```

Последним, о чём хотелось бы рассказать, $-$ `Canvas`. Этот элемент обладает возможностьями низкоуровневой отрисовки, что идеально подходит для создания лепестковой диаграммы. Функция `canvas`, которая создаёт элемент `Canvas`, принимает в качестве аргументов 2 замыкания: `prepaint` нужен для добавления интерактивности, анимаций и сложных вычислений; `paint` занимается уже самой отрисовкой. 
```rust
canvas(
    |bounds: Bounds<Pixels>, window: &mut Window, cx: &mut App| {},
    move |bounds: Bounds<Pixels>, _, window: &mut Window, cx: &mut App| {}
)
```

`Bounds` представляет прямоугольную область в двумерном пространстве, задаваемую начальной точкой и размером. До этого мы нигде не применяли `Window`, сейчас настал его час, так как именно он содержит методы для отрисовки.

Метод `window.paint_path(path, rgb(color))` рисует переданные пути указанным цветом. Для того, чтобы создать пути, по которым окно будет рисовать, применяется структура `PathBuilder`. Метод `PathBuilder::stroke(px(1.0))` говорит, что указанные пути должны быть линиями шириной в 1 пиксель, `PathBuilder::fill()` указывает, что заданные пути являются границей для залитой фигуры. 

Метод `move_to(point)` перемещает текущую точку в заданную. `line_to(point)` строит линию от текущей точки до заданной. Перед тем, как программа закончила рисовать, необходимо закрыть текущий подпуть через `builder.close()`.

Для того, чтобы отрисовать символы применяется метод `paint_glyph`.
```rust
window.paint_glyph(point: Point<Pixels>, font_id: FontId, glyph_id: GlyphId, font_size: Pixels, color: Hsla)
```
Глиф $-$ конкретное очертание символа с учётом стиля текста. Тут `point` $-$ это координаты, где будет отрисовываться глиф, `font_id` $-$ идентификатор шрифта, `glyph_id` $-$ идентификатор глифа, `font_size` $-$ размер шрифта, `text_color` $-$ цвет глифа в формате HSLA.

### Приложение

Наконец перейдём к самому запуску всего, что сделали.

<details>
<summary>Полный код <code>app/Cargo.toml</code></summary>

```toml
[package]
name = "app"
version = "1.2.0"
edition = "2021"
publish = false

[[bin]]
name = "app"
path = "src/main.rs"

[dependencies]
di.workspace = true
ui.workspace = true

gpui.workspace = true
gpui-component.workspace = true
anyhow.workspace = true

```

</details>

Перед запуском необходимо реалмзовать структуру, которая загружает и кэширует внешние ресурсы. Я эту структуру полностью скопировал из примера официального репозитория, так как без этой структуры SVG файлы не загружаются.
```rust
struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        std::fs::read(path)
            .map(Into::into)
            .map_err(Into::into)
            .map(Some)
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        Ok(vec![])
    }
}
```

Приложение создаётся через `Application::new().with_assets(Assets{})`. Для запуска используется метод `run`, который принимает замыкание. В нём сначала создаются глобальные объекты через `cx.set_global(global_struct)`. Напоследок вызывается `cx.open_window()`, где передаются опции окна и замыкание, в котором создаётся экран.
```rust
fn main() {
    let base_dir = std::env::current_dir().unwrap();

    let di = DependencyInjector::init(&base_dir).unwrap();
    let stand_service = di.stand_service();

    Application::new().with_assets(Assets{}).run(move |cx: &mut App| {
        cx.set_global(gpui_component::Theme::default());
        cx.set_global(di);
        cx.set_global(Theme::default());
        cx.set_global(Locale::new());

        let main_state = cx.new(|_cx| {
            MainScreenState::new(stand_service)
        });

        cx.open_window(
            gpui::WindowOptions::default(),
            |_, cx| {
                cx.new(|cx| {
                    MainScreen::new(main_state, cx)
                })
            },
        ).unwrap();
    });
}
```

<details>
<summary>Полный код <code>app/src/main.rs</code></summary>

```rust
use gpui::{
    AssetSource, SharedString, Application, App, AppContext
};
use std::borrow::Cow;
use anyhow::Result;

use di::DependencyInjector;
use ui::{
    MainScreenState, MainScreen, Theme, locale::Locale
};

/// An application asset provider responsible for bridging GPUI's resource management layer with external storage locations.
struct Assets;

impl AssetSource for Assets {
    /// Dynamically loads a resource binary payload from the local filesystem based on its relative path string.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice indicating the unique lookup destination route of the target asset.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(Cow::Owned))` containing the raw file byte array vector data if found,
    ///   `Ok(None)` if the resource file doesn't exist, or an IO `Err` if a filesystem error occurs.
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        std::fs::read(path)
            .map(Into::into)
            .map_err(Into::into)
            .map(Some)
    }

    /// Scans a directory path and indexes all nested asset identities inside it.
    ///
    /// Currently stubbed to return an empty collection since the framework accesses targeted 
    /// media assets explicitly by their exact paths via the `load` method.
    ///
    /// # Arguments
    ///
    /// * `_path` - A root directory path token slice where the recursive scanning would begin.
    ///
    /// # Returns
    ///
    /// * An empty vector array wrapped in a successful `Result`.
    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        Ok(vec![])
    }
}

fn main() {
    let base_dir = std::env::current_dir().unwrap();

    let di = DependencyInjector::init(&base_dir).unwrap();
    let stand_service = di.stand_service();

    Application::new().with_assets(Assets{}).run(move |cx: &mut App| {
        cx.set_global(gpui_component::Theme::default());
        cx.set_global(di);
        cx.set_global(Theme::default());
        cx.set_global(Locale::new());

        let main_state = cx.new(|_cx| {
            MainScreenState::new(stand_service)
        });

        cx.open_window(
            gpui::WindowOptions::default(),
            |_, cx| {
                cx.new(|cx| {
                    MainScreen::new(main_state, cx)
                })
            },
        ).unwrap();
    });
}
```

</details>

# Итог

Общая структура проекта выглядит следующим образом:
```
.
├── assets
│   ├── data
│   │   └── jojo-stands.csv
│   ├── icons
│   │   ├── language.svg
│   │   └── theme.svg
│   ├── images
│   │   ├── 'Achtung Baby.png'
│   │   ├── Aerosmith.png
│   │   ├── ...
│   └── locales
│       ├── en.ftl
│       └── ru.ftl
├── Cargo.lock
├── Cargo.toml
└── crates
    ├── app
    │   ├── Cargo.toml
    │   └── src
    │       └── main.rs
    ├── core
    │   ├── Cargo.toml
    │   └── src
    │       ├── lib.rs
    │       ├── models
    │       │   ├── mod.rs
    │       │   └── stand.rs
    │       ├── repositories
    │       │   ├── mod.rs
    │       │   └── stand.rs
    │       ├── services
    │       │   ├── mod.rs
    │       │   └── stand.rs
    │       └── types
    │           ├── mod.rs
    │           └── rank.rs
    ├── di
    │   ├── Cargo.toml
    │   └── src
    │       ├── dependency_injector.rs
    │       └── lib.rs
    ├── infrastructure
    │   ├── Cargo.toml
    │   └── src
    │       ├── dtos
    │       │   ├── mod.rs
    │       │   ├── rank.rs
    │       │   └── stand.rs
    │       ├── file
    │       │   ├── mod.rs
    │       │   └── path_manager.rs
    │       ├── lib.rs
    │       ├── mappers
    │       │   ├── mapper.rs
    │       │   ├── mod.rs
    │       │   ├── rank.rs
    │       │   └── stand.rs
    │       └── repositories
    │           ├── csv_stand.rs
    │           └── mod.rs
    └── ui
        ├── Cargo.toml
        └── src
            ├── features
            │   ├── main_screen
            │   │   ├── components
            │   │   │   ├── mod.rs
            │   │   │   ├── sidebar.rs
            │   │   │   └── stand_info
            │   │   │       ├── components
            │   │   │       │   ├── mod.rs
            │   │   │       │   ├── radar_chart.rs
            │   │   │       │   └── stand_image.rs
            │   │   │       ├── mod.rs
            │   │   │       └── stand_info.rs
            │   │   ├── mod.rs
            │   │   ├── state.rs
            │   │   └── view.rs
            │   └── mod.rs
            ├── lib.rs
            ├── locale.rs
            ├── shared
            │   ├── button.rs
            │   └── mod.rs
            └── theme.rs
```

Можем смело запустить проект через `cargo run`, и мы получаем следующую картину:

Спасибо за чтение!

[Репозиторий с проектом](https://github.com/ToniTrum/JoJo_Stands_Viewer)