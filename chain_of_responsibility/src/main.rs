mod department;
mod patient;

use department::{Cashier, Department, Doctor, Medical, Reception};
use patient::Patient;

fn main() {
    // chain of responsibility pattern
    // new でつないでいる
    let cashier = Cashier::default();
    let medical = Medical::new(cashier);
    let doctor = Doctor::new(medical);
    let mut reception = Reception::new(doctor);

    let mut patient = Patient {
        name: "John".into(),
        ..Patient::default()
    };

    // Reception -> Doctor -> Medical -> Cashier の順に処理される
    // 処理されると各処理の完了状況が true になる
    reception.execute(&mut patient);

    println!("\nThe patient has been already handled:\n");

    // 2 回目は already done
    reception.execute(&mut patient);
}
