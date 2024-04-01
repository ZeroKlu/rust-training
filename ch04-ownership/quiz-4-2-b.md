## Quiz - Chapter 4.2 b ##

> ---
> **Question 1**<br>
> Consider the permissions in the following program:
>
> ```rust
> let mut s = String::from("Hello");
> let t = &mut s;
> /* here */
> t.push_str(" world");
> println!("{}", s);
> ```
>
> <img src="../additional-files/images/quiz_0402b_img1.png" alt="Question 1 Memory Diagram" title="Question 1 Memory Diagram" style="width:380px"><br>
>
> At the point marked ```/* here */```, what are the 
> permissions on the path ```s```? Select each permission 
> below, or select "no permissions" if the path has no 
> permissions.
>
> > Response<br>
> > ☐ R<br>
> > ☐ W<br>
> > ☐ O<br>
> > ☑ No permissions<br>
> 
> ---
>
> **Question 2**<br>
> Consider the permissions in the following program:
>
> ```rust
> fn get_first(v: &Vec<String>) -> &str {
>     &v[0]
> }
> 
> fn main() {
>     let mut strs = vec![
>         String::from("A"), String::from("B")
>     ];
>     let first = get_first(&strs);
>     if first.len() > 0 {
>         strs.push(String::from("C"));
>     }
> }
> ```
>
> <img src="../additional-files/images/quiz_0402b_img2.png" alt="Question 2 Memory Diagram" title="Question 2 Memory Diagram" style="width:440px"><br>
>
> Which of the following best explains why ```strs``` loses 
> and regains write permissions?
>
> > Response<br>
> > ⦿ ```get_first``` returns an immutable reference to 
> > data within ```strs```, so ```strs``` is not writable
> > while ```first``` is live<br>
> > ⊚ ```strs``` is not writable while the immutable 
> > reference ```&strs``` passed to ```get_first``` is 
> > live<br>
> > ⊚ Because ```first``` refers to ``strs``, then ``strs`` 
> > can only be mutated within a nested scope like the 
> > if-statement<br>
> > ⊚ ```strs``` does not need write permissions until the 
> > ```strs.push(..)``` operation, so it only regains write 
> > permissions at that statement<br>
> 
> ---
>
> **Question 3**<br>
> Consider this unsafe program:
>
> ```rust
> let v1 = vec![1, 2, 3];
> let mut v2 = v1;
> v2.push(4);
> println!("{}", v1[0]); // [L1]
> ```
> 
> <img src="../additional-files/images/quiz_0402b_img3.png" alt="Question 2 Memory Diagram" title="Question 2 Memory Diagram" style="width:200px"><br>
>
> Which of the following best describes the point at which 
> undefined behavior occurs in this program?
>
> > Response<br>
> > ⊚ ```v2``` owns the vector data on the heap, while 
> > ```v1``` does not<br>
> > ⊚ ```v1``` has its pointer invalidated by the push on 
> > line 3<br>
> > ⊚ ```v1``` has been moved into ```v2``` on line 2<br>
> > ⊚ ```v1[0]``` reads ```v1```, which points to 
> > deallocated memory<br>
> 
> ---

![image](../additional-files/images/quiz_0402b1.png)
![image](../additional-files/images/quiz_0402b2.png)
![image](../additional-files/images/quiz_0402b3.png)
