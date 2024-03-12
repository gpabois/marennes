pub struct Size<Unit> {
    width: Unit,
    height: Unit
}

impl<Unit> Default for Size<Unit> where Unit: Default {
    fn default() -> Self {
        Self { width: Default::default(), height: Default::default() }
    }
}

impl<Unit> std::ops::Add for Size<Unit> 
    where Unit: std::ops::Add<Unit, Output=Unit>{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width + rhs.width,
            height: self.height + rhs.height
        }
    }
}

impl<Unit> std::ops::Mul<Unit> for Size<Unit> 
    where Unit: std::ops::Mul<Unit, Output=Unit> + Copy {
    type Output = Self;

    fn mul(self, rhs: Unit) -> Self::Output {
        Self {
            width: self.width * rhs,
            height: self.height * rhs
        }
    }
}

impl<Unit> std::ops::Sub for Size<Unit> 
    where Unit: std::ops::Sub<Unit, Output=Unit>{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            width: self.width - rhs.width,
            height: self.height - rhs.height
        }
    }
}

pub struct Position<Unit> {
    x: Unit,
    y: Unit
}

impl<Unit> std::ops::Add for Position<Unit> 
    where Unit: std::ops::Add<Unit, Output=Unit> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<Unit> std::ops::Sub for Position<Unit> 
    where Unit: std::ops::Sub<Unit, Output=Unit>{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl<Unit> Default for Position<Unit> where Unit: Default {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default() }
    }
}

pub struct Block<Unit> {
    pub size: Size<Unit>,
    pub position: Position<Unit>
}