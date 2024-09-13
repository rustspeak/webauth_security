struct  Person {
    nom : String,
    prenom: String,
    identifiant_unique : String,
    

}
use std::io;
    fn generateur_didentifiant(nombre : i32 ) -> String {
        match nombre {
            '0'..='9' => map( |x| x*5),
            
        }
    }

impl Person {

    fn systeme_authentification(&self) -> Person{

        println!("entrez votre  nom  : ");
        let  mut nom  =  String::new();
        io::stdin().read_line(&mut nom).unwrap();
        let  nom : String  = nom.trim().parse().unwrap();

        println!("entrez votre  prenom : ");
        let  mut prenom  =  String::new();
        io::stdin().read_line(&mut prenom).unwrap();
        let  prenom : String  = prenom.trim().parse().unwrap();

        println!("entrez  votre  identifiant unique : ");
        let  mut id  =  String::new();
        io::stdin().read_line(&mut id).unwrap();
        let  id  = id.trim().parse().unwrap();


        Person {
            nom : nom.to_string(),
            prenom: prenom.to_string(),
            identifiant_unique : id,
        }
    }



}


fn main() {
    let x =  4;
    let    r= generateur_didentifiant(x);
    println!("{:?}", r);
}