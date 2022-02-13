struct Rectangle{
  width: u32,
  height: u32,
}
fn main() {
  let width1 = 30;
  let height1 = 50;
  println!(
    "普通>面积是：{}",
    area(width1, height1)
  );
  let rect1 = (30,40);
  println!(
    "元组>面积是：{}",
    area_t(rect1)
  );
  let rect2 = Rectangle{
    width: 10,
    height: 30,
  };
  println!(
    "结构体>面积是：{}",
    area_s(&rect2)
  );
}
// 普通
fn area(width: u32, height: u32) -> u32 {
  width * height
}
// 元组
fn area_t(dimensions: (u32,u32)) -> u32 {
  dimensions.0 * dimensions.1
}
// 结构体
fn area_s(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}