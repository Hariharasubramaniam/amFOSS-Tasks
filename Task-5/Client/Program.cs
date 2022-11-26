using System.Net;
using System.Net.Sockets;
using System.Text;

// check whether all required namespaces are imported

public class SynchronousSocketClient
{
    private static int bytesRec;
    public static void StartClient()
    {
        // Data buffer for incoming data.  
        byte[] bytes = new byte[1024];

        // Connect to a remote device.  

        try
        {
            // Establish the remote endpoint for the socket.  
            // check if the port is defined or not
            IPHostEntry ipHostInfo = Dns.GetHostEntry(Dns.GetHostName());
            IPAddress ipAddress = ipHostInfo.AddressList[0];
            IPEndPoint remoteEP = new IPEndPoint(ipAddress, 8080);
            // Connect the socket to the remote endpoint. Catch any errors.
            Socket sender = new Socket(ipAddress.AddressFamily, SocketType.Stream, ProtocolType.Tcp);
            // Check whether TCP Socket is created correctly


            // Encode the data string into a byte array.


            // Connect the socket to the remote endpoint. Catch any errors. 

            try
            {

                sender.Connect(remoteEP);
                Console.WriteLine("Socket connected to {0}", sender.RemoteEndPoint.ToString());
                // check if the variable is defined correctly or not
                // Encode the data string into a byte array.
                Console.WriteLine("Enter the Person Name: ");
                string name = Console.ReadLine();
                Console.WriteLine("Enter the Person Intrest: ");
                string intrusts = Console.ReadLine();
                Console.WriteLine("Enter the Person Email: ");
                string mail = Console.ReadLine();
                byte[] msg = Encoding.ASCII.GetBytes(name + "," + intrusts + "," + mail);
                // Send the data to the remote endpoint.
                // Send the data through the socket.
                int bytesSent = sender.Send(msg);
                // Receive the response from the remote device.
                int bytesRec = sender.Receive(bytes);
                Console.WriteLine(Encoding.ASCII.GetString(bytes, 0, bytesRec));
                // Release the socket.
                sender.Close();
                // Encode the data string into a byte array.  
                // check the data type of the data you are sending.
                // Send the data through the socket.  


            }
            catch (ArgumentNullException ane)
            {
                Console.WriteLine("ArgumentNullException : {0}", ane.ToString());
            }
            catch (SocketException se)
            {
                Console.WriteLine("SocketException : {0}", se.ToString());
            }
            catch (Exception e)
            {
                Console.WriteLine("Unexpected exception : {0}", e.ToString());
            }

        }
        catch (Exception e)
        {
            Console.WriteLine(e.ToString());
        }
    }

    // check the main function
    public static int Main(String[] args)
    {
        StartClient();
        return 0;
    }
}
