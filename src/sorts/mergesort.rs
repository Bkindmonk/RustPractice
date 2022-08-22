pub(crate) 
fn mergesort<T:Ord+Copy>( real: &mut[T] , scratch: &mut[T]){
        if real.len() <=1{
            return;
        }
        assert_eq!(real.len(),scratch.len());
    
        let halflength: usize = real.len()/2;
        let realhalf0len:usize;
        let realhalf1len:usize;
    
        { //creating a new scope for temporary borrows
            let realhalves: (&mut [T], &mut [T]) = (*real).split_at_mut(halflength);
            let scratchhalves: (&mut [T], &mut [T]) = (*scratch).split_at_mut(halflength);
    
            mergesort(realhalves.0, scratchhalves.0);
            mergesort(realhalves.1, scratchhalves.1);
    
            realhalf0len = realhalves.0.len();
            realhalf1len = realhalves.1.len();
        } //borrow from realhalves and scratchhalves expires
    
        let mut realpointerleft: usize = 0;
        let mut realpointerright: usize = 0;
        let mut scratchpointer: usize = 0;
    
    
        //merge
        while (realpointerleft < realhalf0len) && (realpointerright < realhalf1len) {
    
            if (*real)[realpointerleft] <= (*real)[realhalf0len+realpointerright]{
                (*scratch)[scratchpointer] = (*real)[realpointerleft];
                realpointerleft +=1;
            }else{
                (*scratch)[scratchpointer] = (*real)[realhalf0len+realpointerright];
                realpointerright+=1;
            }
            scratchpointer+=1;
        }
    
    
        //copy remainder
        while realpointerleft < realhalf0len {
            (*scratch)[scratchpointer] = (*real)[realpointerleft];
            realpointerleft+=1;
            scratchpointer+=1;
        }
    
        while realpointerright < realhalf1len {
            (*scratch) [scratchpointer] = (*real)[realhalf0len+realpointerright];
            realpointerright+=1;
            scratchpointer+=1;
        }
    
        scratchpointer = 0;
        while scratchpointer<realhalf0len+realhalf1len{
            (*real)[scratchpointer] = (*scratch)[scratchpointer];
            scratchpointer+=1;
        }
    
    
    }