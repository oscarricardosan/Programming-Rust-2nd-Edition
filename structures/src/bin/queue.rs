pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            println!("Reasigno jobs");
            std::mem::swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        // Método que toma propiedad de self, desinicializando las propiedades
        // older y younger al retornarlas.
        (self.older, self.younger)
    }

    pub fn new() -> Self {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }
}

fn main() {
    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };

    q.push('0');
    q.push('1');

    //Lee la cola de older, esta vacía, voltea younger en older
    assert_eq!(q.pop(), Some('0'));

    q.push('8');
    assert_eq!(q.pop(), Some('1'));

    //Lee la cola de older, esta vacía por los dos anteriores pop, luego voltea younger en older
    assert_eq!(q.pop(), Some('8'));

    //Lee la cola de older, esta vacía al igual que younger, retorna None
    assert_eq!(q.pop(), None);

    assert!(q.is_empty());
    q.push('9');
    assert!(!q.is_empty());

    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };
    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');

    let (older, younger) = q.split();
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);

    // Ejecutar assert!(q.is_empty());
    // fallara, ya que self.older y self.younger
    // pasaron a ser propiedad de older y younger.

    //Ejemplo de Queue como puntero Box, no hay que modificar el código
    // de la estructura, rust toma automaticamente la referencia
    let mut bq = Box::new(Queue::new());
    bq.push('6');

    // Queue de números
    let mut bi = Queue::new();
    bi.push(1);
    assert_eq!(bi.pop(), Some(1));

    println!("Finalización exitosa");
}
