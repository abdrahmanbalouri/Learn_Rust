#[derive(Debug, PartialEq, Eq)]
pub enum Jacket {
    Black, White , Flowers
    
}
#[derive(Debug, PartialEq, Eq)]
pub enum Hat{
    Snapback, Baseball, Fedora
    
}


#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}

pub fn choose_outfit(formality_level: Option<u32>, invitation_message: Result<&str, &str>) -> Outfit {
  let k = match formality_level{

    None => Jacket::Flowers,
    Some( a) if a>0 => Jacket::White,
    _=> Jacket::Black,
  };

  let l =  match invitation_message{

    Ok(_)=>Hat::Fedora,
    _ =>  Hat::Snapback,
  } ;


  if formality_level.is_none() && l == Hat::Snapback {
  return  Outfit {
        jacket : Jacket::Flowers,
        hat: Hat::Baseball,
    };
  }
return Outfit{
    jacket: k,
    hat :l,
};
}