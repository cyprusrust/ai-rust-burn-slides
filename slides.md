---
# try also 'default' to start simple
theme: seriph
# random image from a curated Unsplash collection by Anthony
# like them? see https://unsplash.com/collections/94734566/slidev
background: /rust.jpg
# apply any windi css classes to the current slide
class: 'text-center'
# https://sli.dev/custom/highlighters.html
# show line numbers in code blocks
lineNumbers: false
# some information about the slides, markdown enabled
info: |
  ## Backend Services in Rust
  
# persist drawings in exports and build
colorSchema: 'dark'
drawings:
  persist: false
# use UnoCSS
css: unocss
---

# Backend Services in <span class="text-orange-300">Rust</span>
<div class="pt-12">
  <span @click="$slidev.nav.next" class="px-2 py-1 rounded cursor-pointer" hover="bg-white bg-opacity-10">
  </span>
</div>

<div class="abs-br m-6 flex gap-2">
  <button @click="$slidev.nav.openInEditor()" title="Open in Editor" class="text-xl icon-btn opacity-50 !border-none !hover:text-white">
    <carbon:edit />
  </button>
  <a href="https://github.com/cyprusrust/backend-services-rust" target="_blank" alt="GitHub"
    class="text-xl icon-btn opacity-50 !border-none !hover:text-white">
    <carbon-logo-github />
  </a>
</div>

<style>
  h1 {
    margin-top: 100px
  }
</style>

---

# Who are you?
<style>
  .slidev-layout {
    background: #145DA0;
  }
  .slidev-layout h1 {
    color: #fff;
  }
  .avatar {
    height: 180px;
    width: 180px;
    border-radius: 90px;
    background: #7f8c8d url(/framp.jpg) no-repeat;
    background-clip: padding-box;
    background-size: 180px;
  }
  .cyprusrust {
    height: 180px;
    background: #0e1a22 url(/cyprus-rust.png) no-repeat;
    background-position: center center;
    background-clip: padding-box;
    background-size: 180px;
  }
  .slidev-layout a:hover {
    color: #fff;
  }
</style>

<div class="flex justify-center">
  <div class="flex flex-col self-end -mb-7">
    <div class="avatar" />
    <a href="https://framp.me" target="_blank">framp.me</a>
    <span>Federico Rampazzo</span>
  </div>
  <div class="flex flex-col">
    <apiplantLogo />
    <a href="https://apiplant.com" class="mx-8" target="_blank">apiplant.com</a>
    <div class="m-8 mb-0 w-64 rounded-3xl rounded-bl-0 cyprusrust" />
    <a href="https://cyprusrust.org" class="mx-8" target="_blank">cyprusrust.org</a>
  </div>
</div>

<!--
Our team at has plenty of experience in Python and JavaScript and it was common for us to just create a new project in Django or Fastify. We kept experimenting with Rust over the years and we feel that now the ecosystem is mature enough and that the benefits of using Rust outweigh the larger ecosystem that other languages offer.

Our new default choice for when we start a new project is to reach out for Rust!
-->

---
layout: two-cols
---

<template v-slot:default>

# Why Rust?
- Growing ecosystem
- Most loved language
- High level language
- Performance

</template>
<template v-slot:right>

```rust
#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
}

impl Person {
  fn say_hello(&self) {
    println!("DEBUG: {:?}", self);
    println!("My name is {} and I'm {} years old", 
      self.name, self.age);
  }
}
```

```rust
let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let new_arr: Vec<i32> = arr.iter()
  .map(|x| x * 2)
  .filter(|x| x % 3 == 0)
  .fold(Vec::new(), |mut acc, x| {
    let last = acc.pop().unwrap_or_default();
    acc.push(last);
    acc.push(last + x);
    acc
  });
println!("{:?}", new_arr);
```

</template>

<!--
Why Rust?

Rust is growing and there are a lot of high quality dependencies available. It's the been the most loved language in the Stack Overflow survey for 7 years!
I would compare it to Node.js in 2014. Sure, we still don't have automated CMS and fancy frameworks but there are serious foundations which allow you to be productive.

On the technical side, Rust is a high level language, it offers all the functional methods and syntactic sugar you may want. You have first class function, iterators, futures (or promises), async / await syntax, meta-programming and OOP classes.
For example this struct here contains a string, a number and we can implement any methods we want on it (static or not). That command starting with a hash symbol is a macro, a powerful piece of code which will get expanded and replaced with more complicated code. This allow us to think one layer up and not have to write a lot of boilerplate.
So it's a very high level language; at the same time it is also a low level language, offering you exceptional performance and asking you to think about how you are using the memory.

How does Rust reconcile these two aspects?
For once Rust frequently uses the concept of zero cost abstractions. For example, this iterative methods in many high level languages would be executed sequentially with 3 different loops. In Rust they get compiled to a single loop and are as fast as a less declarative for loop.
-->

---
layout: two-cols
---

<template v-slot:default>

# Why Rust?

- Type Safety
- Borrow Checker


</template>
<template v-slot:right>

```rust
// FIX ME
use std::thread;

let data = vec![1, 2, 3];

let handle1 = thread::spawn(move || {
    for i in data.iter() {
        println!("Thread 1: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
});

let handle2 = thread::spawn(move || {
    for i in data.iter() {
        println!("Thread 2: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
});

handle1.join().unwrap();
handle2.join().unwrap();
```



</template>

<!--
But performance is not the only trick up of Rust's sleeve. The main reason I love writing Rust is safety. The type system is incredibly powerful and allow you, and the dependencies you use, to express a series of constraints which make it hard to produce incorrect code which compile.
Rust also has an innovative system, the borrow checker, which keep tracks of data ownership at compile time. In this way you're preventing runtime errors and you don't pay a performance price in production.

Our experience is that you generally spend more time writing your code and less time chasing bugs. To make a comparison, with Typescript we got close but we didn't get the same benefits: between dependencies with no or partial types or a developer who really needed to ship something and decided to skip the types, we were never really sure whether a codebase was reliable or not.

Let's have a look at this example: 
cd ./demo
cargo run

It doesn't compile

Look at the error rustc --explain E0382

Bada bum bada bam, it works with Arc
-->

---

# Architecture deep dive

https://github.com/apiplant/anonpaste-backend

<br>

Dependencies:
- Axum - web application framework, middleware based
- sqlx - SQL abstraction layer
- dotenv, anyhow, thiserror - handy utilities you want in your app

<style>
  .slidev-layout {
    background: #9b59b6;
  }
  .slidev-layout h1 {
    color: #fff;
  }

  .slidev-layout a {
    color: #fff;
  }
</style>

<!--
But let's get to the meat: I want to show you around a simple production application so that you can walk out of here and think "Cool, maybe I don't know Rust but with a bit of extra time I *could* do that instead of picking my framework of choice!"
You can find the code on github for later and you'll find these slides as well in an incoming blog post on cyprusrust.org.

As usual we're sitting on the shoulders of OSS giants: axum is the web framework, sqlx is a layer to connect to a pool of SQL connections. 

Show main

Show server

Show routers

Show models

Show tests
-->

---
layout: two-cols
---

<template v-slot:default>

# Database Integration

```rust
let paste = sqlx::query_as!(
    Paste,
    "SELECT id, 
            content, 
            expiry_time, 
            expiry_views 
        FROM paste WHERE id = ?",
    id,
)
.fetch_one(&mut *conn)
.await?;
```

```rust
async fn view_paste_handler(
    Path(id): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<Paste>, Error> {
    let mut conn = app_state.pool.acquire().await?;
    let paste = Paste::view(&mut conn, id).await?;
    Ok(Json(paste))
}
```

</template>
<template v-slot:right>

# &nbsp;


```rust
sqlx::query!(
    "INSERT INTO paste ( id, content, 
      expiry_time, expiry_views )
        VALUES ( ?1, ?2, ?3, ?4)",
    payload.id,
    payload.content,
    payload.expiry_time,
    payload.expiry_views
)
.execute(conn)
.await?;
```

<div class="p-2 text-dark-100">
What happen if I put an non existant column or I try to use the wrong type for a bind?
</div>
</template>

<!--
Let's zoom in on some snippets so you can get a better idea of what's going on in a typical request.

The resource receive the request and calls the model.

The model is using an sqlx macro to query our Sqlite database and return the data as an instance of our type.

The resource receive the data and serialise the response back

Because sqlx is insanely cool, the SQL query is typechecked at compile time, so if you try to write the wrong query you'll get a compile error.

Let's try.

-->

---

# Serde and types

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paste {
    pub id: String,
    pub content: String,
    pub expiry_time: Option<i64>,
    pub expiry_views: Option<i64>,
}

serde_json::to_string(&paste_payload)
serde_json::from_slice(&body).unwrap()

async fn view_paste_handler(
    Path(id): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<Paste>, Error> {
    let mut conn = app_state.pool.acquire().await?;
    let paste = Paste::view(&mut conn, id).await?;
    Ok(Json(paste))
}
```

<!--
We've seen how much we're serialising and deserialising but we haven't seen exactly how. This is because serialisation and deserialisation is almost effortless, thanks to serde, another amazing library.
For most types we can use the #derive macro to tell the compiler we want that type to be serialisable. 
If a struct is made of types that can be automatically serialised, that's all you need to do.
In this example I'm even using another macro to tell serde to serialise the properties in camelCase instead of snake_case (which is the rust default).
In this way my frontend application will receive camelCase objects and won't even know about the snakes hiding in the backend.

Axum has an implementation of Json which allow to return as response anything that can be serialised and that deserialise incoming payloads, turning them in the correct type.
-->

---
layout: two-cols
---

<template v-slot:default>

# Error handling

```rust
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("NO_AUTHORIZATION")]
    Unauthorized,
    #[error("NO_PERMISSION")]
    Forbidden,
    #[error("NOT_FOUND")]
    NotFound,
    #[error("INTERNAL_DB_ERROR")]
    Sqlx(sqlx::Error),
    #[error("INTERNAL_ERROR")]
    Anyhow(#[from] anyhow::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::Sqlx(err),
        }
    }
}
```

</template>

<template v-slot:right>

# &nbsp;
```rust
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            self.status_code(),
            Json(ErrorMessage {
                msg: self.to_string(),
            }),
        )
            .into_response()
    }
}
```

</template>

<!--
Error handling is one of my favourite part of this codebase. Thanks to the powerful type systems and sprinkle of awesome dependencies defining your own error is as simple as creating an Enum.
We can't force all our dependencies to use our errors, though.
Luckily there is no need for that: Rust has a From trait which allow us to convert certain types in others. 
The crate thiserror even provides a macro to automatically generate a From implementation, but we may want to do it ourselves to handle special case.

In this example I'm checking whether sqlx couldn't find a row and helpfully convert into a Not Found error.

In order for our error to work with axum, we need to implement this trait IntoResponse which convert our error in a status code and error message
-->

---
layout: two-cols
---

<template v-slot:default>

# Testing

```rust
#[tokio::test]
async fn fetch_paste() {
    let config = &get_test_config();
    let (router, app_state) = get_app(config).await.unwrap();
    let mut conn = app_state.pool.acquire().await.unwrap();
    Paste::create(
        &mut conn,
        CreatePaste {
            id: "test-id".to_string(),
            content: "Hello".to_string(),
            expiry_time: None,
            expiry_views: None,
        },
    )
    .await
    .unwrap();
}
```

</template>

<template v-slot:right>

# &nbsp;

```rust
let response = router
    .with_state(app_state)
    .oneshot(
        Request::builder()
            .uri("/api/paste/test-id")
            .body(Body::empty())
            .unwrap(),
    )
    .await
    .unwrap();

assert_eq!(response.status(), StatusCode::OK);
let response = response.into_body();
let body = body::to_bytes(response).await.unwrap();
let body: Value = serde_json::from_slice(&body).unwrap();
assert_eq!(
    body,
    json!({
      "content": "Hello".to_string(), 
      "id": "test-id".to_string(),
      "expiryTime": Null, 
      "expiryViews": Null
    })
);
```
</template>

<!--


Let's have a look at tests

We're getting the test configuration, creating our app and preparing the database with some data.

Then we're building a request and firing it. 
Lastly we use serde to check the body is what we expect.

-->

---

# Deploy to fly.io

```bash
flyctl deploy # generate a failing build and configuration
flyctl volumes create db -s 1 #1GB
flyctl secrets set EMAIL_FROM=info@anonpaste.pw
flyctl secrets set EMAIL_NAME=AnonPaste
flyctl secrets set SENDGRID_API_KEY=SECRET
flyctl secrets set DATABASE_URL=sqlite:/mnt/db/production.sqlite?mode=rwc
flyctl deploy 
```


```ini
# setup the volume we creataed
[mount]
source = "db"
destination = "/mnt/db"
```

<!--
Good!
Now that our app is tested and ready to be deployed, we can turn to fly.io

Fly.io makes it relatively easy to deploy your app and they even have a generous free tier which allow you to run up to three apps, each with 1GB of storage.

The codebase has a Dockerfile and trying to deploy with fly.io should generate the correct configuration in a fly.toml file.
Nothing will work though as we need secrets and a storage for our Sqlite database.

Once that's all set and done we can deploy again and this time we should have a Rust app running in the cloud!

-->

---

# Links

&nbsp;

All the slides will be published on https://cyprusrust.org/blog

Come and say hi on the CDC Discord: https://cdc.cy

<div class="m-8 mb-0 w-3/4 h-3/5 mx-auto rounded-3xl cyprusrust" />


<style>
  .cyprusrust {
    background: #0e1a22 url(https://cyprusrust.org/assets/cyprus-rust.png) no-repeat;
    background-position: center center;
    background-clip: padding-box;
    background-size: 55%;
  }
</style>


<!--
That's all I've got! 
I'll publish the slides over the weekend and please come and say hi in the discord! :)

Thank you!
-->
