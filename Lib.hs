module Lib (withPoint, getX, getY) where

import Foreign.C.Types
import Foreign.Ptr

data Point = Point

foreign import ccall "new_point" newPoint :: CInt -> CInt -> IO (Ptr Point)

foreign import ccall "delete_point" deletePoint :: Ptr Point -> IO ()

foreign import ccall "get_x" getX :: Ptr Point -> CInt 

foreign import ccall "get_y" getY :: Ptr Point -> CInt 

withPoint :: CInt -> CInt -> (Ptr Point -> IO a) -> IO a
withPoint x y f = do
  cpoint <- newPoint x y
  result <- f cpoint
  deletePoint cpoint
  pure result
