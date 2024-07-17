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
  ## Burn framework and AI Rust
  
# persist drawings in exports and build
colorSchema: 'dark'
drawings:
  persist: false
# use UnoCSS
css: unocss
---

# <span class="text-orange-300">AI</span> in <span class="text-orange-300">Rust</span> and the <span class="text-orange-300">Burn</span> framework
<div class="pt-12">
  <span @click="$slidev.nav.next" class="px-2 py-1 rounded cursor-pointer" hover="bg-white bg-opacity-10">
  </span>
</div>

<div class="abs-br m-6 flex gap-2">
  <button @click="$slidev.nav.openInEditor()" title="Open in Editor" class="text-xl icon-btn opacity-50 !border-none !hover:text-white">
    <carbon:edit />
  </button>
  <a href="https://github.com/cyprusrust/burn-ai-in-rust" target="_blank" alt="GitHub"
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
    background: #7f8c8d url(/framp.png) no-repeat;
    background-clip: padding-box;
    background-size: 180px;
  }
  .cyprusrust {
    height: 150px;
    background: #0e1a22 url(/cyprus-rust.png) no-repeat;
    background-position: center center;
    background-clip: padding-box;
    background-size: 150px;
  }
  .cdc {
    height: 150px;
    background: #0e1a22 url(/cdc-logo.svg) no-repeat;
    background-position: center center;
    background-size: 120px;
  }
  .slidev-layout a:hover {
    color: #fff;
  }
</style>

<div class="flex justify-center">
  <div class="flex flex-col self-end -mb-7 px-8">
    <div class="avatar" />
    <a href="https://framp.me" target="_blank">framp.me</a>
    <span>Federico Rampazzo</span>
  </div>
  <div class="flex flex-col">
    <apiplantLogo />
    <a href="https://apiplant.com" class="mx-8" target="_blank">apiplant.com</a>
    <div class="flex">
      <div class="flex flex-col">
        <div class="m-8 mb-0 w-64 rounded-3xl rounded-bl-0 cyprusrust" />
        <a href="https://cyprusrust.org" class="mx-8" target="_blank">cyprusrust.org</a>
      </div>
      <div class="flex flex-col">
        <div class="m-8 mb-0 w-64 rounded-3xl rounded-bl-0 cdc" />
        <a href="https://cdc.cy" class="mx-8" target="_blank">cdc.cy</a>
      </div>
    </div>
  </div>
</div>

<!--
My name is Federico Rampazzo, my company is called API Plant and I focus on AI consulting.

I'm also one of the organizers of the Cyprus Developer Community and the founder of Cyprus Rust user group!
-->

---
theme: seriph
class: 'text-center'
css: unocss
---
# &nbsp;
# &nbsp;
# &nbsp;
# &nbsp;

# Why use Rust for AI?


<!--
Today I want to talk about my experience using Rust instead of Python to do Generative AI projects.

Why would you want to use Rust for AI?
-->
---

# Why? 

<div class="flex gap-9xl mx-auto w-148 my-8">
  <ul>
    <li>Strong Static Typing</li>
    <li>Ownership System</li>
    <li>More safety - Less bugs</li>
  </ul>
  <ul>
    <li>Easy deployment</li>
    <li>Fearless Concurrency</li>
    <li>Great tooling</li>
  </ul>
</div>

<img src="/tweet.png" />

<!--
Rust has an amazing type system which allow you to express ideas very well.
The emphasis on safety and memory ownership helps you catch a lot of bugs at runtime.

I find the resulting codebase to be great to maintain at the cost of making prototyping slower!

Safe concurrency is another great perk to have:
I once deployed a tensorflow based solution in Rust to do image classification - I then was able to scale the code synchronising downloads, api calls and inference across multiple threads and eventually to process 400M images on a small cluster.

Deployment is easy, you can compile everything to a single binary and be done. 

You also don't have to deal with 2000 different package managers; cargo, Rust's package manager, works pretty well.
  
-->


---
layout: two-cols
---

<template v-slot:default>

# My experience
 - 2 Gen AI projects in Rust
   - Preparing datasets
   - Text model finetuning
   - Image generation
   - Retrieval Augmented Generation (RAG)
# &nbsp;

</template>
<template v-slot:right>

# &nbsp;

 - 2 Gen AI Python project
   - Preparing datasets
   - Text generation
   - Audio generation
   - RAG

 - 1 GenAI JavaScript (+ Python) project
   - Preparing datasets
   - Vercel AI
   - Text model finetuning
<style>
  .slidev-layout {
    background-image: url('/rust-vs-python.png');
    background-size: 40%;
    background-repeat: no-repeat;
    background-position: 10% 100%;
  }
</style>

</template>

<!--
In the last year I did 2 projects using Rust, 2 in Python and 1 in JavaScript.

Then, even if Elon Musk doesn't want me to...
-->

---

# Training 3 AGIs in my spare time

<video autoplay loop src="/agi.mp4" />


<!--
I trained 3 AGIs in my spare time

** fake laughs from the audience **
-->

---
layout: two-cols
---

<template v-slot:default>

# My experience
 - 2 Gen AI projects in Rust
   - Preparing datasets
   - Text model finetuning
   - Image generation
   - Retrieval Augmented Generation (RAG)
# &nbsp;

</template>
<template v-slot:right>

# &nbsp;

 - 2 Gen AI Python project
   - Preparing datasets
   - Text generation
   - Audio generation
   - RAG

 - 1 GenAI JavaScript / Python project
   - Preparing datasets
   - Vercel AI
   - LangchainJS
   - Text model finetuning

<style>
  .slidev-layout {
    background-image: url('/rust-vs-python.png');
    background-size: 40%;
    background-repeat: no-repeat;
    background-position: 10% 100%;
  }
</style>
</template>

<!--
Going back to my experience, I'll start with JavaScript.


Preparing datasets required doing manual scripts, there are not many fancy libraries to manipulate datasets.
The Vercel AI documentation needs some work and I was not impressed by LangChainJS support vs LangChain (but things might have changed since then).
Overall I don't see any reasons to choose JS, unless that's the only language your team knows.

Using Python meant dealing with a lot of setup issues and debugging buggy dependencies upstream.
On the plus side, everything you might need is available with a Python API.

Rust was definitely harder to start with but I ended up with codebases that are easier to maintain.
In some cases I had to scavenge github for an algorithm implementation; sometimes I had to port it from the equivalent python library.
But overall they were solvable problems and allowed me to work in a codebase I could trust.

-->

---
layout: two-cols
---

<template v-slot:default>

# Data Preparation

- [polars](https://pola.rs/) (instead of pandas)
- [ndarray](https://github.com/rust-ndarray/ndarray) (instead of numpy)
- [linfa](https://rust-ml.github.io/linfa/) (instead of scikit)
- [tiktoken](https://github.com/openai/tiktoken), [tokenizers](https://github.com/huggingface/tokenizers)
- [safetensors](https://github.com/huggingface/safetensors)


</template>
<template v-slot:right>

<img src="/polars.png" />
<img src="/linfa.png" />

</template>

<!--
On the data preparation side, Rust shines.
Oftentimes you end up with pretty complicated code to generate or extract data from somewhere else and being able to create nice abstractions is great.

Polars is a great alternative to Pandas, I started using it in place of Pandas on Python too!

ndarray is comparable to numby;

Linfa is nice when the algorithm you want has been implemented but there are a few holes compared to scikit.

Tokenizers commonly used from Python are written in Rust, so they work out of the box.

The same is true for safetensors (the format for tensors).

-->
---


# Model inference - Candle

https://github.com/huggingface/candle


 - Similar API to PyTorch without the torchlib
 - Support for quantized models
 - Support LoRA
 - Runs on wasm (CPU) on the web, run on serverless
 - Examples with popular models:
    - LLama, OLMo, Mistral, Gemma, Qwen
    - StableDiffusion
    - Whisper
    - YOLO, SAM and more

### Limits:
 - Inference is not as optimised as PyTorch on CUDA
 - Backpropagation is supported but focus is not on training

<style>
  .slidev-layout {
    background: #145DA0;
  }
  .slidev-layout h1 {
    color: #fff;
  }

  .slidev-layout a {
    color: #fff;
  }
</style>

<!--

For model inference, candle is a great minimalist framework

It has a similar API to Pytorch but it's implemented completely in Rust
It supports quantized models and LoRa

It can run on wasm on cpu in the browser
It has sample implementation for all models you want to run

Performance is good on CPU and Apple Silicon, for CUDA there is still work to do.

Doing training is possible, as backpropagation is supported, but there is little documentation / code


-->

---


# Model inference & training - Burn

https://github.com/Tracel-Ai/burn

 - Great API to define a model for both inference and training
 - Define a stream of tensor operations + auto optimizations
 - Support multiple backends with the same API
    - tch-rs (based on torchlib, fastest for cuda)
    - wgpu 
        - runs on gpu, faster than cpu, slower than cuda
        - runs on WebGPU in the browser, faster than wasm
    - candle!
    - ndarray (cpu)
 - Custom file format

### Limits:
 - Not a lot of examples, a few unsupported features
 - No official supports for importing safetensors (yet)

<style>
  .slidev-layout {
    background: #F55D14;
    background-image: url('/burn.webp');
    background-repeat: no-repeat;
    background-position: 90% 90%;
  }
  .slidev-layout h1 {
    color: #fff;
  }

  .slidev-layout a {
    color: #fff;
  }
</style>


<!--
Burn is a different type of framework, it's an ambitious approach to ML frameworks.

You define a stream of tensor operations and the framework optimize them for you.

You can still do custom optimization by extending backends.

Switching from training to inference with the same model definition is painless.

It has its own file format which is as safe as safetensors and can be configured to be more or less compressed (based on your usecase) and use Rust type system to convert to the correct precision. 

It also supports importing models from pickle and onnx... but not from safetensors (YET).

Backends can be swapped at runtime.

It supports torchlib, so you get the same performance of PyTorch, it supports candle and its gpu optimisation or ndarray which runs everywhere on cpu.

It even has a wgpu backend which is a popular gpu abstraction library in Rust. The performance is not as good as CUDA but there are proposals to improve the situation. wgpu can also be deployed in the browser and from not very scientific tests, it's roughly 5-10x faster than using CPU and the fastest way to run inference in a browser.

The main cons, especially compared to candle is the availability of examples and code available for it.

-->
---

# Burn training CLI UI

<img src="/burn-train.png" />

<!--
Burn also has a nice CLI UI for training. This would be the tensorboard equivalent
-->

---

# Burn model 
<img src="/code-conv.png" class="w-144 mx-auto" />

<!--
This is a sample model, implementing a convolutional network for image classification

One interesting thing to note is that the Module derive generates code to make your model trainable.
The type of our Model is generic on the type of backend used and we can see it's using 3 convolutional blocks, 2 linear layers, one dropout layer and uses gaussian error linear units as the activation function.
The instances and their configuration for each of these layers will be part of the constructor of our model.

-->
---

# Burn model 
<img src="/code-llama.png" class="w-192 mx-auto" />

<!--
This instead is a high level implementation of llama.
You can see the token embedding which turns token in vectors, the stack of layers with multi headed attention and residual connections which output our token vectors. You can see we are also passing:
 - A mask which will be used to mask tokens generated after the current token
 - A rotary encoder which is a clever way to encode the position of tokens as rotations in paired complex number
 - A normalization layer (RMS - Root Mean Square)

Thanks to Rust powerful type system, understanding what's happening behind the scene is much easier.

-->

---

# Links

&nbsp;

All the slides will be published on https://cyprusrust.org/blog 

and on https://cyprusrust.github.io/ai-rust-burn-slides

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
I'll publish the slides and please come and say hi in the discord! :)

Thank you!
-->
