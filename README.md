# beecrowd-problems
*A simple repository containing some of my beecrowd problem solving*
---

I mainly started to solve some of beecrowd problems in order to learn a new programming language, **rust**.\
In this repository i've made some tools to help me solve these problems faster. These tools are the two following bash scripts 

[My beecrowd profile](https://www.beecrowd.com.br/judge/pt/profile/536645)

---

### **new.sh**

Take as argument **[1]** The category of the problem and **[2]** the problem number.\
Simply create the desired problem file as a rust file with a basic code template.\

Example: ./new.sh 1 1000

```rust
use std::io;

fn main() {

}
```

### **run.sh**

Take as argument **[1]** The category of the problem and **[2]** the problem number.\
Simply compile and run the desired problem file with the rustc compiler.\

Example: ./run.sh 1 1000