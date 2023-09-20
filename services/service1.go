package services

type Service1 struct {
	services *Services
}

func (s *Service1) DoSomething() {
	s.services.Service2.DoSomething()
	s.services.Service2.Query()

	s.services.SomeString = "A new string"
}
