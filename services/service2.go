package services

type Service2 struct {
	services *Services
}

func (s *Service2) DoSomething() int {
	// fmt.Println("Doing something from Service2")
	return 1
}

func (s *Service2) Query() {
	// fmt.Println("Querying from Service2")
}
