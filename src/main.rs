use std::thread;


fn main() {
    
    let vetor = vec![15, 16, 18, 21, 1, 1, 1, 1, 1, 10, 12, 14];
    
    let mut menor = 0;
    
    let mut primeiro = 0;
    let mut ultimo = vetor.len()-1;
    let mut meio = (primeiro+ultimo)/2;
        
    loop {
        meio = (primeiro+ultimo)/2;
        if vetor[meio-1] > vetor[meio]{
            menor = meio;
            break;
        }
        if vetor[meio+1] < vetor[meio]{
            menor = meio + 1;
            break;
        }

        println!("{} {} {}", primeiro, meio, ultimo);
        thread::sleep(std::time::Duration::from_millis(500));
        if vetor[primeiro] <= vetor[meio]{
            primeiro = meio;
        } else {
            ultimo = meio;
        }
        
    }
    
    println!("{} {}", menor, vetor[menor]);
}