trait FromAll =  for<T> From<T>;  
trait FromAllWithSendAndSync =  for<T> From<T> where T : Send + Sync; 
