use sigmaos_archwiki::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_archwiki::new("sigmaos-archwiki").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-archwiki"));
}
