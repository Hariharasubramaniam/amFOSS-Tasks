import 'package:flutter/material.dart';

void main() => runApp(MyApp());

class bunnyApp extends State<MyApp> {
  double y_value = 0;
  double x_value = 0;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        body: Stack(
          children: <Widget>[
            build_bgpic(context),
            build_bunny(context),
            getButtons(context),
          ],
        ),
      ),
    );
  }

  Widget build_bgpic(BuildContext context) {
    return Positioned(
        top: 0,
        right: 0,
        child: Transform.scale(
            scale: 1.5,
            child: Align(
                alignment: Alignment.bottomCenter,
                child: Image.asset(
                  "assets/background.png",
                ))));
  }

//done
  Widget build_bunny(BuildContext context) {
    return Align(
      alignment: Alignment.bottomCenter,
      child: Image.asset(
        "assets/bunny.png",
        width: 200,
        height: 200,
      ),
    );
  }

  Widget getButtons(BuildContext context) {
    return Stack(children: [
      Align(
        alignment: const Alignment(-0.75, 0.75),
        child: TextButton(
          child: const Text("▲", textScaleFactor: 2),
          onPressed: () async {
            setState(() {
              y_value = y_value + 5;
            });
          },
        ),
      ),
      Align(
        alignment: const Alignment(-0.75, 0.90),
        child: TextButton(
          child: const Text("▼", textScaleFactor: 2),
          onPressed: () async {
            setState(() {
              y_value = y_value - 5;
            });
          },
        ),
      ),
      Align(
        alignment: const Alignment(-0.825, 0.825),
        child: TextButton(
          child: const Text("◄", textScaleFactor: 2),
          onPressed: () async {
            setState(() {
              x_value = x_value - 5;
            });
          },
        ),
      ),
      Align(
        alignment: const Alignment(-0.675, 0.825),
        child: TextButton(
          child: const Text("►", textScaleFactor: 2),
          onPressed: () async {
            setState(() {
              x_value = x_value + 5;
            });
          },
        ),
      ),
    ]);
  }
}

class MyApp extends StatefulWidget {
  @override
  bunnyApp createState() => bunnyApp();
  const MyApp({super.key});
}
