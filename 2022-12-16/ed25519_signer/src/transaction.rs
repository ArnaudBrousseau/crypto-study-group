//! Inspired by the snippets at https://willcrichton.net/notes/type-level-programming/
//! This file is a type-level state machine for blockchain transactions
use ed25519_dalek::keypair::Keypair;
struct Initialized;
struct Constructed;
struct Signed;
struct Broadcasted;
struct Confirmed;


#[repr(transparent)]
struct Transaction<State> {
    unsigned_payload: Vec<u8>,
    signing_key: Keypair,
    signature: Vec<u8>,
    signed_payload: Vec<u8>,
    chain_hash: &str,
    num_confirmations: u8,
    _state: PhantomData<State>
}


// Methods for the state are uniquely associated with only the state
impl Transaction<Initialized> {
  // recv consumes ownership, ensuring old state is invalidated
  fn construct(mut self) -> (Transaction<Constructed>, String) {
    let msg = self.chan.recv();
    
    // Type transition to the next step (Constructed)
    (unsafe { transmute(self) }, msg)
  }
}

impl Channel<Sending> {
  fn send(mut self, msg: String) -> Channel<Receiving> {
    self.chan.send(msg);
    unsafe { transmute(self) }
  }
}

#[test]
fn channel_test() {
  let c: Channel<Sending> = Channel::new();
  let c: Channel<Receiving> = c.send("hi");
  let (c, msg) = c.recv();
  // and so on
}
